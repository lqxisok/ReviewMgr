function refresh_render(){
    ele = document.getElementById("total-tex-content");
    renderMathInElement(ele, {
        // customised options
        //  onload="renderMathInElement(document.body);"
        // • auto-render specific keys, e.g.:
        delimiters: [
            {left: '$$', right: '$$', display: true},
            {left: '$', right: '$', display: false},
            {left: '\\(', right: '\\)', display: false},
            {left: '\\[', right: '\\]', display: true}
        ],

        // • rendering keys, e.g.:
        throwOnError : false
    });
}

function prevent_a_link() {
    const anchors = document.querySelectorAll('a[href^="#"]');

    // 为每个锚点链接添加点击事件监听器
    anchors.forEach(anchor => {
        anchor.addEventListener('click', function(event) {
            // 阻止链接的默认跳转行为
            event.preventDefault();
            scroll_container = document.getElementById("total-tex-content");
            // 使用 `scrollIntoView` 方法滚动到特定元素
            target_ele = scroll_container.querySelector(this.getAttribute('href'));
            scroll_container.scrollTo(0, target_ele.offsetTop - scroll_container.offsetTop)

            // 你可以在这里添加其他想要执行的代码
            // 比如手动滚动到某个位置，或者其他逻辑
        });
    });
}
