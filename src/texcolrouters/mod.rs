mod strucs;
mod route_params;

// Struct for passing parameters between routers
use route_params::ReviewProjectParams;

// Struct for passing parameters
use strucs::Project;
use strucs::ProjectIdArgs;
use strucs::CreateProjectArgs;
use strucs::SleepArgs;
use strucs::CreateReviewArgs;
use strucs::Review;
use strucs::ProjectContentSaveArgs;
use strucs::ReviewFromProjectIdArgs;
use strucs::ReviewWindowArgs;
use strucs::WindowNameArgs;
use strucs::DeleteReviewIdArgs;


use leptos::{*, leptos_dom::logging::console_log};
use leptos_router::*;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::JsValue;
use web_sys::{SubmitEvent, MouseEvent};

use super::utils_js::invoke;
use super::utils_js::refresh_render;
use super::utils_js::prevent_a_link;

use leptos_icons::*;
use web_sys::window;


#[component]
pub fn RootRouter() -> impl IntoView {
    view! {
      <Routes>
        <TexColHome/>
        <TexColProject/>
        <TexColCreateReview />
      </Routes>
    }
}


#[component(transparent)]
pub fn TexColHome() -> impl IntoView {
  view!{
    <Route path="/" view=TexColHomeView/>
  }
}

fn open_folder_by_id(id: i32) {
    spawn_local(async move {
        let args = to_value(&ProjectIdArgs { id: id }).unwrap();
        let res = invoke("texcol_open_project_folder_by_id", args).await;
        let is_open = res.as_bool().unwrap();
        if is_open {
            console_log(format!("open folder by id: {} success", id).as_str());
        } else {
            if let Some(window) = window() {
                let _ = window.alert_with_message("Project path is renamed or removed. Please check!");
            }
        }
    });
}

fn delete_project_by_id(id: i32) {
    spawn_local(async move {
        let args = to_value(&ProjectIdArgs { id: id }).unwrap();
        let res = invoke("texcol_delete_project_by_id", args).await;
        let is_delete = res.as_bool().unwrap();
        if is_delete {
            console_log(format!("delete project by id: {} success", id).as_str());
        } else {
            if let Some(window) = window() {
                let _ = window.alert_with_message("Project path is renamed or removed. Please check!");
            }
        }

    });
}

fn delete_review_by_id(review_id: i32) {
    spawn_local(async move {
        let args = to_value(&DeleteReviewIdArgs { review_id: review_id }).unwrap();
        let res = invoke("texcol_delete_review_by_id", args).await;
        let is_delete = res.as_bool().unwrap();
        if is_delete {
            console_log(format!("delete review by id: {} success", review_id).as_str());
        } else {
            if let Some(window) = window() {
                let _ = window.alert_with_message("Review is renamed or removed. Please check!");
            }
        }
    });
}


#[component]
fn TexColHomeView() -> impl IntoView {
    let (show_project_id, set_show_project_id) = create_signal(0);
    let (_is_sqlite_exist, set_is_sqlite_exist) = create_signal(false);
    let (project_tex_inner_html, set_project_tex_inner_html) = create_signal(String::from("Project Content"));
    let (edit_content, set_edit_content) = create_signal(String::from("Project Content"));

    let (edit_status, set_edit_status) = create_signal(false);

    // review signals
    let (all_reviews, set_all_reviews) = create_signal(
        vec![
        Review {
            id: 101,
            project_id: 100,
            status: true,
            reviewer: "Demo Reviewer".to_string(),
            description: "Demo review. ".to_string(),
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }]);

    // content title signal
    let (all_projects, set_all_projects) = create_signal(
        vec![Project {
            id: 100,
            name: "Demo Project".to_string(),
            description: "Description".to_string(),
            tex_path: "default".to_string(),
            bib_path: "default".to_string(),
            proj_path: "proj default".to_string(),
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }]
    );

    let (show_project, set_show_project) = create_signal(Project {
        id: 100,
        name: "demo".to_string(),
        description: "Description".to_string(),
        tex_path: "default".to_string(),
        bib_path: "default".to_string(),
        proj_path: "proj default".to_string(),
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
    });

    // get all reviews
    let update_all_reviews_by_id = move || {
        let related_project_id = show_project_id.get_untracked();
        spawn_local(async move {
            let args = to_value(&ReviewFromProjectIdArgs { proj_id: related_project_id }).unwrap();
            let res = invoke("texcol_get_all_reviews_for_project", args).await;
            let reviews: Vec<Review> = serde_wasm_bindgen::from_value(res.clone()).unwrap();
            if reviews.len() > 0 {
                set_all_reviews.set(reviews);
            } else {
                console_log(format!("get all reviews by id: {:?} success", reviews).as_str());
            }
            
        });
    };

    // save edit
    let save_edit = move |ev: MouseEvent| {
        ev.prevent_default();
        let get_project_id = show_project_id.get_untracked();

        let get_edit_content = edit_content.get_untracked();
        spawn_local(async move {
            let args = to_value(&ProjectContentSaveArgs { id: get_project_id, content: get_edit_content}).unwrap();
            let result: JsValue = invoke("texcol_update_raw_tex_content_by_id", args).await;
            let html_str = result.as_string().unwrap();
            console_log(&html_str);
        });
    };

    // edit view 
    let update_edit_status = move |_ev| {
        set_edit_status.update(|value| *value = !*value);
        let editable = edit_status.get_untracked();
        let get_project_id = show_project_id.get_untracked();

        if editable {
            spawn_local(async move {
                // todo add a new api to get project information by id
                let args = to_value(&ProjectIdArgs { id: get_project_id }).unwrap();
                let result: JsValue = invoke("texcol_read_raw_tex_content_by_id", args).await;
                let html_str = result.as_string().unwrap();
                set_edit_content.set(html_str.clone());
            });
        } else {
            spawn_local(async move {
                // todo add a new api to get project information by id
                let args = to_value(&ProjectIdArgs { id: get_project_id }).unwrap();
                let result: JsValue = invoke("telcol_convert_tex_to_html_by_id", args).await;
                let html_str = result.as_string().unwrap();
                set_project_tex_inner_html.set(html_str.clone());
                refresh_render();
                prevent_a_link();
            });
        }

    };
    
    // update content title
    let update_content_title = move || {
        let get_project_id = show_project_id.get_untracked();
        spawn_local(async move {
            // todo add a new api to get project information by id
            let args = to_value(&ProjectIdArgs { id: get_project_id }).unwrap();
            let project: JsValue = invoke("texcol_get_project_by_id", args).await;
            let project: Project = serde_wasm_bindgen::from_value(project.clone()).unwrap();
            set_show_project.update(move |f| {*f = project;});
        });
    };

    let update_content_project_tex = move || {
        let get_project_id = show_project_id.get_untracked();
        spawn_local(async move {
            // todo add a new api to get project information by id
            let args = to_value(&ProjectIdArgs { id: get_project_id }).unwrap();
            let result: JsValue = invoke("telcol_convert_tex_to_html_by_id", args).await;
            let html_str = result.as_string().unwrap();
            set_project_tex_inner_html.set(html_str.clone());
            refresh_render();
            prevent_a_link();
        });
    };

    let update_projects_info = move || {
        spawn_local(async move {
            let args: JsValue = JsValue::null();
            let results = invoke("texcol_get_all_projects", args).await;
            let projects: Vec<Project> = serde_wasm_bindgen::from_value(results.clone()).unwrap();
            if projects.len() > 0 {
                set_all_projects.set(projects);
                set_show_project_id.set(all_projects.clone().get_untracked()[0].id);
                update_content_title();
                update_content_project_tex();
                update_all_reviews_by_id();
            }
        });
    };

    let check_sql_path = move || {
        spawn_local(async move {
            let args: JsValue = JsValue::null();
            let results: JsValue = invoke("texcol_app_dir", args).await;
            let is_exist = results.as_bool().unwrap();
            if is_exist {
                set_is_sqlite_exist.set(true);
                update_projects_info();
            } else {
                set_is_sqlite_exist.set(false);
            }
        });
    };
    check_sql_path();

    // Create Project bundle
    let open_create_project_window = move |_ev: MouseEvent| {
        spawn_local(async move {
            let args: JsValue = JsValue::null();
            let _results = invoke("create_project_window", args).await;
            loop {
                let args: JsValue = JsValue::null();
                let results = invoke("check_create_project_window_status", args).await;
                let is_visible = results.as_bool().unwrap();
                if !is_visible {
                    break;
                }
                let args = to_value(&SleepArgs { ms: 50 }).unwrap();
                invoke("texcol_sleep_ms", args).await;
            }
            update_projects_info();
        });
    };

    let create_review_for_project = move |_ev: MouseEvent| {
        spawn_local(async move {
            let args = to_value(&ReviewWindowArgs { proj_id: show_project_id.get_untracked() }).unwrap();
            let _results = invoke("create_review_window", args).await;
            // check the review window is visible ?
            loop {
                let args = to_value( &WindowNameArgs { window_name: "create_review".to_string() }).unwrap();
                let results = invoke("check_window_status_by_name", args).await;
                let is_visible = results.as_bool().unwrap();
                if !is_visible {
                    break;
                }
                let args = to_value(&SleepArgs { ms: 50 }).unwrap();
                invoke("texcol_sleep_ms", args).await;
                console_log("sleep 50ms")
            }
            update_all_reviews_by_id();
        });
    };

    view! {
        <div class="home-container">
            <div id="sidebar-container">
                <div id="top-function-area">
                    <h1>"ReviewMgr"</h1>
                </div>
                <div id="sidebar">
                    <For
                        each=move || all_projects.get()
                        key=|state| state.name.clone()
                        let:child
                    >
                        <a on:click=move |_ev| { 
                            set_show_project_id.set(child.id);
                            update_content_title();
                            update_content_project_tex();
                            update_all_reviews_by_id();
                        }
                         class="project-box" class:active=move || {child.id == show_project_id.get()}>
                            <h4>{child.name}</h4>
                            <p>{child.created_at.clone().format("%Y-%m-%d").to_string()}</p>
                        </a>
                        <div class="project-box-icon-bar">
                            <a on:click=move |_| {open_folder_by_id(child.id)}><Icon class="icon-item" icon=icondata::BsFolder /></a>
                            // <a on:click=move |_| {open_folder_by_id(child.id)}><Icon class="icon-item" icon=icondata::BiExportRegular /></a>
                            <a on:click=move |_| {
                                delete_project_by_id(child.id);
                                update_projects_info();
                            }><Icon class="icon-item" icon=icondata::AiDeleteOutlined /></a>
                        </div>
                    </For>
                </div>
                
                <div id="bottom-function-area">
                    <button id="create_btn" on:click=open_create_project_window>"Create Project"</button>
                </div>
            </div>

            <div class="content-container">
            <div id="content-head-box">
            <h1>{move || show_project.get().name}</h1>
                <div class="function-bar">
                    <a id="refresh-katex-render" on:click=update_edit_status >
                        <Icon class="icon-item" icon=icondata::FiEdit3/>
                    </a>
                </div>
            </div>
                <div id="content-box">
                    <div class="tex-content" id="total-tex-content" class:active=move || {!edit_status.get()} inner_html=project_tex_inner_html>
                    </div>
                    <div class="edit-content" id="total-tex-edit-content" class:active=move || {edit_status.get()}>
                        <textarea class="edit-area" placeholder="Tex content"
                        prop:value={move || {edit_content.get()}}
                        on:input=move |ev| {set_edit_content.set(event_target_value(&ev))} >
                        </textarea>
                        <div class="edit-function-bar">
                            <Icon class="icon-item" icon=icondata::RiRefreshSystemLine />
                            <button id="edit-save-btn" on:click=save_edit> Save </button>
                        </div>
                    </div>
                </div>
                <div id="command-box">
                    // <input type="text" id="command-input" placeholder="Type a message..."/>
                    <textarea id="command-input" name="story" rows="2" cols="33" placeholder="Type a message..." readonly>
                    "It was a dark and stormy night..."
                    </textarea>
                    <div class="command-function-bar">
                        <Icon class="icon-item" icon=icondata::RiRefreshSystemLine />
                        <button id="command-submit-btn"> Submit </button>
                    </div>
                </div>
            </div>
            <div class="review-container">
                <div class="review-header">
                    <h3>"Review List"</h3>
                </div>
                <div class="review-content">
                    <div class="all-reviews-contents">
                        <For
                            each=move || all_reviews.get()
                            key=|state| state.id.clone()
                            let:child
                        >
                            <div class="review-item">
                                <div class="review-item-header">
                                    <h4>{child.reviewer}</h4>
                                    <p>{child.created_at.clone().format("%Y-%m-%d").to_string()}</p>
                                </div>
                                <div class="review-item-content">
                                    <p>{child.description}</p>
                                </div>
                                <div class="review-item-footer">
                                    <button class="review-item-btn">"Edit"</button>
                                    <button class="review-item-btn review-delete-btn" 
                                    on:click=move |_ev| {
                                        delete_review_by_id(child.id);
                                        update_all_reviews_by_id();
                                    }>"Delete"</button>
                                </div>
                            </div>
                        </For>
                    </div>
                </div>
                <div class="review-footer" on:click=create_review_for_project>
                    <button id="create_review_btn">"Create Review"</button>
                </div>
                
            </div>
        </div>
    }
}


#[component(transparent)]
pub fn TexColProject() -> impl IntoView {
  view!{
    <Route path="/project/create" view=TexColProjectView/>
  }
}

#[component]
fn TexColProjectView() -> impl IntoView {
    let (project_name, set_project_name) = create_signal(String::new());
    let (select_file_path, set_select_file_path) = create_signal(String::new());
    let (select_bib_file_path, set_select_bib_file_path) = create_signal(String::new());
    let (select_dir_path, set_select_dir_path) = create_signal(String::new());

    let select_tex_file = move |_ev| {
        spawn_local(async move {
            let args = JsValue::null();
            let path = invoke("texcol_select_file", args).await.as_string().unwrap();
            set_select_file_path.set(path);
        });
    };

    let select_bib_file = move |_ev| {
        spawn_local(async move {
            let args = JsValue::null();
            let path = invoke("texcol_select_bib_file", args).await.as_string().unwrap();
            set_select_bib_file_path.set(path);
        });
    };

    let select_dir = move |_ev| {
        spawn_local(async move {
            let args = JsValue::null();
            let path = invoke("texcol_select_dir", args).await.as_string().unwrap();
            set_select_dir_path.set(path);
        });
    };

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_project_name.set(v);
    };

    let create_project = move |ev: SubmitEvent| {
        ev.prevent_default();
        
        spawn_local(async move {
            let project_name = project_name.get_untracked();
            let select_file_path = select_file_path.get_untracked();
            let select_bib_file_path = select_bib_file_path.get_untracked();
            let select_dir_path = select_dir_path.get_untracked();
            if project_name.is_empty() {
                return;
            }
            if select_file_path.is_empty() {
                return;
            }
            let args = to_value(&CreateProjectArgs { 
              project_name: &project_name, 
              tex_path: &select_file_path,
              bib_tex_path: &select_bib_file_path,
              dir_path: &select_dir_path,
            }).unwrap();
            let _new_msg = invoke("texcol_create_project", args).await.as_string().unwrap();
            let args = JsValue::null();
            let _ = invoke("close_create_project_window", args).await;
        });
    };

    view! {
        <div class="row">

            <div class="create-project-container" on:submit=create_project>
                <h1>"Create Project"</h1>
                <form id="create-project-form">
                    <div class="form-group">
                        <label for="project-name">Project Name:</label>
                        <input type="text" id="project-name" on:input=update_name name="projectName" required/>
                    </div>
                    <div class="form-group">
                        <label for="file-upload">"Select a Tex File:"</label>
                        <input type="button" on:click=select_tex_file id="select_file" value="Select a Tex file" />
                    </div>
                    <div class="form-group">
                        <label for="bib-file-upload">"Select a Bib File:"</label>
                        <input type="button" on:click=select_bib_file id="select_file" value="Select a Bib file" />
                    </div>
                    <div class="form-group">
                        <label for="project-dir">"Select a Project Dir:"</label>
                        <input type="button" on:click=select_dir id="select_dir" value="Select a directory" />
                    </div>
                    <button type="submit">"Create Project"</button>
                </form>
            </div>

        </div>
    }
}

#[component(transparent)]
pub fn TexColCreateReview() -> impl IntoView {
    view!{
    <Route path="/review/:proj_id/create" view=TexColCreateReviewView/>
    }
}

#[component]
fn TexColCreateReviewView() -> impl IntoView {

    let project_params = use_params::<ReviewProjectParams>();
    let (reviewer, set_reviewer) = create_signal(String::from("Reviewer"));
    let (description, set_description) = create_signal(String::from("Review Content"));

    let update_review = move |ev| {
        let v = event_target_value(&ev);
        set_reviewer.set(v);
    };

    // convert the id to usize
    let id = move || {
        project_params.with(|params| {
            params.as_ref()
                .map(|params| params.proj_id)
                .unwrap_or_default()
        })
    };

    let create_review = move |ev: SubmitEvent| {
        ev.prevent_default();
        let proj_id = id().unwrap() as i32;
        let reviewer = reviewer.get_untracked();
        let description = description.get_untracked();

        spawn_local(async move {
            let args = to_value(&CreateReviewArgs { 
              proj_id: proj_id, 
              status: true,
              reviewer: reviewer,
              description: description,
            }).unwrap();
            let _new_msg = invoke("texcol_create_review_for_project", args).await.as_string().unwrap();
            console_log(&format!("[Debug Msg]: {}", _new_msg));
            let args = JsValue::null();
            let _res = invoke("close_review_window", args).await;
        });
    };

    view! {
        <div class="create-review-container">
            <form id="create-review-form" on:submit=create_review>
                <h1>{move || format!("Create Review for {}", id().unwrap())}</h1>

                <div class="review-form-group">
                    <label for="reviewer-name">"Reviewer Name:"</label>
                    <input type="text" id="project-name" on:input=update_review name="ReviewerName" required/>
                </div>
                <div class="review-form-group" id="review-form-content">
                    <label for="review-create-content">"Review Content"</label>
                    <textarea id="review-create-content" name="ReviewContent" 
                    prop:value={move || {description.get()}}
                    on:input=move |ev| {set_description.set(event_target_value(&ev))}
                    required></textarea>
                </div>
                <button type="submit" id="create-review-btn">"Create Review"</button>
            </form>
        </div>

    // </div>
    }
}

