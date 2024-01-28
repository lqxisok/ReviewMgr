/// Preprocess the tex file to fit the requirments of the converter
/// 
/// 1. Remove all comments
/// 2. Replace all renewcommand
/// 3. Only retain the title for the paper
/// 

pub mod util;
pub mod file_utils;
pub mod analysis;
pub mod ast;
pub mod parse;
pub mod display_source;
pub mod emit;

use display_source::*;
use ast::BibEntry;
use std::{rc::Rc};

use file_utils::{read_file, change_CRLF_to_LF};
use analysis::Analysis;
use ast::NodeLists;
use emit::emit_to_string;
use ropey::Rope;

// Add this import statement
use indoc::eprintdoc;
use nom::combinator::complete;
use nom::Offset;
use std::path::Path;
use std::process;
use parse::{bib, document};

use nom::{
    bytes::complete::{tag, take_till, take_until},
    character::complete::{newline, char},
    sequence::{preceded, delimited, tuple},
    IResult,
};

pub type Error<'a> = nom::error::Error<&'a str>;
pub type Result<'a, O> = IResult<&'a str, O, Error<'a>>;

pub fn consumed_slice<'a>(before: &'a str, after: &'a str) -> &'a str {
    assert!(after.len() <= before.len());
    let len = before.len() - after.len();
    debug_assert_eq!(&before[len..], after);
    &before[..len]
}

fn remove_comments_for_preamble(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, matched) = take_till(|c| c == '\\')(input)?;

    let (input, _) = tuple((
        tag("\\documentclass"),
        parse_options,
        parse_value,
    ))(input)?;

    // let (input, _) = take_till(|c| c == '%')(input)?;
    // let (input, _) = preceded(tag("%"), take_till(|c| c =='\n'))(input)?;


    let (input, lines) = nom::multi::many0(
        nom::sequence::tuple((
            take_till(|c| c == '%'),
            preceded(tag("%"), take_till(|c| c =='\n')),
            newline,
        )),
    )(input)?;
    let lines: Vec<_> = lines.into_iter().map(|(_, line, _)| line).collect();

    Ok((input, lines))
}


fn parse_options(input: &str) -> IResult<&str, &str> {
    delimited(
        char('['),
        take_until("]"),
        char(']'),
    )(input)
}

fn parse_title(input: &str) -> IResult<&str, &str> {
    let (input, _) = take_until("\\title")(input)?;
    let (input, _) = tag("\\title")(input)?;
    parse_value(input)
}

fn parse_value(input: &str) -> IResult<&str, &str> {
    delimited(
        char('{'),
        take_until("}"),
        char('}'),
    )(input)
}

fn paper_content_env(input: &str) -> IResult<&str, &str> {
    let (input, _) = take_until("\\begin{document}")(input)?;
    let (input, _) = tag("\\begin{document}")(input)?;

    let (input, paper_content) = take_until("\\end{document}")(input)?;
    let (input, _) = tag("\\end{document}")(input)?;
    Ok((input, paper_content))
}


pub fn regenerate_all_tex_content(input: &str) -> Rc<str> {
    let (input, title) = match parse_title(input) {
        Ok((input, title)) => (input, title),
        Err(_) => (input, ""),
    };
    
    let (input, paper_content) = match paper_content_env(input) {
        Ok((input, paper_content)) => (input, paper_content),
        Err(_) => (input, ""),
    };
        
    let new_formated_tex_string = format!(r"\documentclass[10pt,twocolumn,letterpaper]{{article}}
\title{{{}}}
\begin{{document}}{}\end{{document}}", title, paper_content);
    Rc::from(new_formated_tex_string)
}


pub fn parse_source<'a, O>(
    parser: impl FnMut(&'a str) -> parse::Result<'a, O>,
    source: &'a str,
    source_path: &'a Path,
) -> O {
    match complete(parser)(source) {
        Ok((_, o)) => o,
        Err(nom::Err::Incomplete(_)) => panic!(),
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
            //let location = remaining_begin_location(e.input, source);
            let location = Location(source.offset(e.input), source.offset(e.input) + 1);
            let location_display = SourceDisplay {
                source,
                location,
                source_path: Some(source_path),
                underlined: true,
            };
            eprintdoc! {"
                Error: Unexpected token
                {location_display}
            "};
            process::exit(1);
        }
    }
}

pub fn tex_rope_to_html(tex_rope: &Rope, bib_source: Vec<BibEntry<'_>>
    , source_path: &str) -> String {
    let content_string = tex_rope.to_string();
    let tex_path_obj = Path::new(source_path);
    let doc_resouce = parse_source(document, &content_string, tex_path_obj); 
    
    let node_lists = NodeLists::new(&doc_resouce);
    let analysis = Analysis::new(&doc_resouce, &bib_source, &node_lists);
    
    let emit_string = emit_to_string(&doc_resouce, &analysis);
    emit_string
}

pub fn process_bib_file<'a>(bib_src: &'a str, bib_path_obj: &'a Path ) -> Vec<BibEntry<'a>> {
    let bib_entries = parse_source(bib, bib_src, bib_path_obj);
    bib_entries
}

pub fn latex_to_html_string(tex_path: &str, bib_path: &str) -> String {
    // Parse the tex files.
    let tex_path_obj = Path::new(tex_path);
    let input = read_file(tex_path_obj);
    let input = change_CRLF_to_LF(input.unwrap());
    let result = regenerate_all_tex_content(input.as_str());
    let doc_resouce = parse_source(document, &result, tex_path_obj); 

    let bib_path_obj = Path::new(bib_path);
    let bib_src = read_file(bib_path_obj);
    let bib_src = change_CRLF_to_LF(bib_src.unwrap());
    let bib_entries = parse_source(bib, bib_src.as_str(), bib_path_obj);

    // Generate lists of nodes and analyze the bib/latex asts.
    let node_lists = NodeLists::new(&doc_resouce);
    let analysis = Analysis::new(&doc_resouce, &bib_entries, &node_lists);

    let emit_string = emit_to_string(&doc_resouce, &analysis);
    emit_string
}

#[cfg(test)]
mod tests {
    use std::{path::{Path, PathBuf}, str::FromStr};
    use super::*;

    #[test]
    fn test_remove_comments() {
        let input: &str = r#"
        asdasd
        % CVPR 2024 Paper Template; see https://github.com/cvpr-org/author-kit
        
        \documentclass[10pt,twocolumn,letterpaper]{article}
        
        % askdhkajshdkasjhd
        % askdhkajshdkasjhdasd
        % askdhkajshdkasjhdasddsaf
        this is not a commant 
        % bbbbbbbbbb
        "#;

        let expected = vec!["\n        ", "\\documentclass[10pt,twocolumn,letterpaper]{article}\n", "\n        "];

        let (_, lines) = remove_comments_for_preamble(input).unwrap();
        // assert_eq!(lines, expected);
    }

    #[test]
    fn parse_options_test() {
        let input: &str = r#"[10pt,twocolumn,letterpaperasdasdas\asdasd]{article}
        "#;
        let (_, result) = parse_options(input).unwrap();
    }

    #[test]
    fn parse_value_test() {
        let input: &str = r#"{10pt,twocolumn,letterpaperasdasdas\asdasd}
        "#;
        let (_, result) = parse_value(input).unwrap();
    }

    #[test]
    fn parse_title_test() {
        let input: &str = r#"
        asdasd
        % CVPR 2024 Paper Template; see https://github.com/cvpr-org/author-kit
        
        \documentclass[10pt,twocolumn,letterpaper]{article}
        
        % askdhkajshdkasjhd
        % askdhkajshdkasjhdasd
        % askdhkajshdkasjhdasddsaf
        this is not a commant 
        \title{the title of this paper is aaaa}
        "#;
        let (_, title) = parse_title(input).unwrap();
    }

    #[test]
    fn paper_content_env_test() {
        let input: &str = r#"
        asdasd
        % CVPR 2024 Paper Template; see https://github.com/cvpr-org/author-kit
        
        \documentclass[10pt,twocolumn,letterpaper]{article}
        % askdhkajshdkasjhd
        % askdhkajshdkasjhdasd
        % askdhkajshdkasjhdasddsaf
        this is not a commant 
        \title{the title of this paper is aaaa}
        \begin{document}
        \maketitle

        this is the content of the paper
        asdasd
            asdasd

            fasdasd

            asfa
            \begin{figure}
            \end{figure}
            asdasd
            afasfsd

        \end{document}
        "#;
        let (_, content) = paper_content_env(input).unwrap();
    }

    #[test]
    fn regenerate_all_tex_content_test() {

        let input = "./test2.tex";
        let bib_src = "./main.bib";
        let emit_string = latex_to_html_string(input, bib_src);
        println!("{}", emit_string)

    }


}