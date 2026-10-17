// The Rythme website: the burger menu, the copy buttons and the remembered language.
// Every page works without this file: the links then stay visible under the logo.
(() => {
  "use strict";
  const root = document.documentElement;

  // ----- burger menu -----
  const burger = document.querySelector(".burger");
  const nav = document.getElementById("nav");
  const wide = window.matchMedia("(min-width: 860px)");

  function setMenu(open) {
    root.classList.toggle("menu-open", open);
    burger.setAttribute("aria-expanded", String(open));
    // Closed on a phone, the panel's links leave the tab order.
    if (nav) nav.inert = !open && !wide.matches;
  }

  if (burger && nav) {
    burger.hidden = false;
    setMenu(false);
    burger.addEventListener("click", () => setMenu(!root.classList.contains("menu-open")));
    nav.addEventListener("click", (ev) => {
      if (ev.target.closest("a")) setMenu(false);
    });
    document.addEventListener("keydown", (ev) => {
      if (ev.key === "Escape" && root.classList.contains("menu-open")) {
        setMenu(false);
        burger.focus();
      }
    });
    wide.addEventListener("change", () => setMenu(false));
  }

  // ----- the language chosen with the switch, for the next visit of / -----
  for (const link of document.querySelectorAll(".lang__link[data-lang]")) {
    link.addEventListener("click", () => {
      document.cookie = `rythme-lang=${link.dataset.lang}; Path=/; Max-Age=31536000; SameSite=Lax`;
    });
  }

  // ----- copy buttons -----
  if (navigator.clipboard) {
    for (const button of document.querySelectorAll("[data-copy]")) {
      const label = button.textContent;
      button.hidden = false;
      button.addEventListener("click", async () => {
        try {
          await navigator.clipboard.writeText(button.dataset.copy);
          button.textContent = button.dataset.copied;
          setTimeout(() => { button.textContent = label; }, 1600);
        } catch {
          // The clipboard may be refused: the command stays selectable.
        }
      });
    }
  }
})();
