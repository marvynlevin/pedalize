addEventListener("load", () => {
    const burger = document.querySelector("#navbar .burger-menu");
    const buttons = document.querySelector("#navbar .buttons");
    const nav = document.getElementById("navbar");

    burger.addEventListener("click", () => {
        buttons.classList.toggle("active");
        nav.classList.toggle("active");
    });

    addEventListener("resize", () => {
        if (window.innerWidth > 1240) {
            buttons.classList.remove("active");
            nav.classList.remove("active");
        }
    })
});

