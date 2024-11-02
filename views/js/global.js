window.addEventListener("load", () => {
    // select
    document.querySelectorAll("details.select").forEach((select) => {
        let summary = select.querySelector("summary");

        select.querySelectorAll("li").forEach((e) => {
            e.addEventListener("click", () => {
                summary.textContent = e.textContent;
                select.removeAttribute("open")
            })
        })

    })
})

document.addEventListener("click", (e) => {
    let selects = document.querySelectorAll("details.select").values();
    selects.forEach((s) => {
        if (e.target !== s) {
            s.removeAttribute("open")
        }
    })
})