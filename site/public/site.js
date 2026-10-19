// The Rythme website: the burger menu, the language menu, the copy buttons, the remembered language, the
// installable app (the service worker, the "new version" banner, the install button
// and the hint for iOS), and the color themes. Every page works without this file: the links then
// stay visible under the logo, and the site is a plain website.
(() => {
  "use strict";
  const root = document.documentElement;

  // ----- burger menu -----
  const burger = document.querySelector(".burger");
  const nav = document.getElementById("nav");
  const wide = window.matchMedia("(min-width: 860px)");

  // The bottom of the bar, where the menu panel and the language list start: the bar
  // wraps on two lines when the text is enlarged, and scrolls away on a short screen.
  const bar = document.querySelector(".bar");
  const measureBar = () => {
    if (bar) root.style.setProperty("--bar-bottom", `${Math.max(0, Math.round(bar.getBoundingClientRect().bottom))}px`);
  };
  window.addEventListener("resize", measureBar);

  function setMenu(open) {
    if (open) measureBar();
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

  // ----- the language menu of the bar -----
  // A `details`: it opens and closes without this script; this closes it with Escape
  // (the focus goes back to its button), a click elsewhere, the focus leaving it, or the
  // burger menu opening.
  const langMenu = document.querySelector(".lang--bar");
  if (langMenu) {
    const toggle = langMenu.querySelector("summary");
    const close = () => { langMenu.open = false; };
    langMenu.addEventListener("keydown", (ev) => {
      if (ev.key === "Escape" && langMenu.open) {
        ev.stopPropagation();
        close();
        toggle.focus();
      }
    });
    document.addEventListener("click", (ev) => {
      if (langMenu.open && !langMenu.contains(ev.target)) close();
    });
    langMenu.addEventListener("focusout", (ev) => {
      if (ev.relatedTarget && !langMenu.contains(ev.relatedTarget)) close();
    });
    langMenu.addEventListener("toggle", () => {
      if (langMenu.open) measureBar();
      if (langMenu.open && burger && root.classList.contains("menu-open")) setMenu(false);
    });
    if (burger) burger.addEventListener("click", close);
  }

  // ----- the language chosen with the switch, for the next visit of / -----
  for (const link of document.querySelectorAll(".lang__link[data-lang]")) {
    link.addEventListener("click", () => {
      // The cookie stays under the site's base path (`/Rythme` on GitHub Pages).
      const path = link.pathname.slice(0, link.pathname.lastIndexOf(`/${link.dataset.lang}/`)) || "/";
      document.cookie = `rythme-lang=${link.dataset.lang}; Path=${path}; Max-Age=31536000; SameSite=Lax`;
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

  // ----- the installable app -----
  const store = {
    get(k) { try { return localStorage.getItem(k); } catch { return null; } },
    set(k, v) { try { localStorage.setItem(k, v); } catch { /* storage may be blocked */ } },
  };
  const standalone = () => window.matchMedia("(display-mode: standalone)").matches || navigator.standalone === true;
  const $ = (id) => document.getElementById(id);

  // The service worker keeps the site offline. A new version installs in the
  // background (looked for at each visit, and each time the app comes back to the
  // foreground) and waits. It comes in by itself only when nobody can lose anything:
  // when the app opens or comes back and the new version is ready before the visitor
  // has touched anything, or when the app goes to the background with no field being
  // edited and no program running (an installed app is rarely closed, so "next launch"
  // would otherwise never come). Otherwise the banner offers to reload, and nothing
  // reloads on its own: the playground may be in use.
  const site = document.querySelector(".site[data-sw]");
  const update = $("app-update");
  if (site && update && "serviceWorker" in navigator) {
    let registration = null;
    let reloading = false;
    // Touched since the page opened or came back to the foreground.
    let touched = false;
    for (const type of ["pointerdown", "keydown", "input"]) document.addEventListener(type, () => { touched = true; }, true);
    const editing = () => {
      const el = document.activeElement;
      return (!!el && (el.isContentEditable || /^(INPUT|TEXTAREA|SELECT)$/.test(el.tagName))) || document.body.classList.contains("running");
    };
    const offer = () => { update.hidden = false; };
    const apply = () => {
      reloading = true;
      registration.waiting.postMessage("skip-waiting");
    };
    // A new version is waiting: in by itself if that loses nothing, else the banner.
    const settle = () => {
      if (!registration || !registration.waiting || !navigator.serviceWorker.controller || reloading) return;
      const hidden = document.visibilityState === "hidden";
      if (!editing() && (hidden || !touched)) apply();
      else offer();
    };
    const watch = (worker) => {
      worker.addEventListener("statechange", () => {
        if (worker.state === "installed") settle();
      });
    };
    window.addEventListener("load", async () => {
      try {
        registration = await navigator.serviceWorker.register(site.dataset.sw, { scope: site.dataset.scope, updateViaCache: "none" });
      } catch {
        return;
      }
      if (!registration) return;
      settle();
      if (registration.installing) watch(registration.installing);
      registration.addEventListener("updatefound", () => watch(registration.installing));
      // An app left open looks for a new version when it comes back, and takes the one
      // waiting when it leaves.
      document.addEventListener("visibilitychange", () => {
        if (document.visibilityState === "visible") {
          touched = false;
          registration.update().catch(() => {});
        } else {
          settle();
        }
      });
    });
    navigator.serviceWorker.addEventListener("controllerchange", () => {
      if (reloading) location.reload();
    });
    $("app-reload").addEventListener("click", () => {
      reloading = true;
      if (registration && registration.waiting) registration.waiting.postMessage("skip-waiting");
      else location.reload();
    });
    $("app-later").addEventListener("click", () => { update.hidden = true; });
  }

  // The install button, when the browser offers it (Chrome, Edge…), until installed.
  const install = $("app-install");
  let prompt = null;
  if (install) {
    const item = install.closest("li");
    window.addEventListener("beforeinstallprompt", (ev) => {
      ev.preventDefault();
      prompt = ev;
      item.hidden = standalone();
    });
    install.addEventListener("click", async () => {
      if (!prompt) return;
      prompt.prompt();
      await prompt.userChoice.catch(() => null);
      prompt = null;
      item.hidden = true;
    });
    window.addEventListener("appinstalled", () => {
      prompt = null;
      item.hidden = true;
    });
  }

  // iPhone and iPad have no install prompt: once, until closed, how to add the site to
  // the home screen.
  const hint = $("ios-hint");
  const ios = /iPad|iPhone|iPod/.test(navigator.userAgent) || (/Macintosh/.test(navigator.userAgent) && navigator.maxTouchPoints > 1);
  if (hint && ios && !standalone() && !store.get("rythme-ios-hint")) {
    hint.hidden = false;
    $("ios-hint-close").addEventListener("click", () => {
      hint.hidden = true;
      store.set("rythme-ios-hint", "closed");
    });
  }

  // ----- the color themes -----
  // The inline script of the head has already applied the theme kept (before the first
  // paint); this shows the picker and keeps the next choice. Auto has no `data-theme`:
  // the system's light or dark applies.
  const picker = document.querySelector(".nav__item--theme");
  const toggle = $("theme-toggle");
  if (picker && toggle) {
    const THEME_KEY = "rythme-theme";
    const radios = [...picker.querySelectorAll('input[name="theme"]')];
    const metas = [...document.querySelectorAll('meta[name="theme-color"]')];
    const apply = (theme) => {
      const radio = radios.find((r) => r.value === theme) || radios[0];
      radio.checked = true;
      if (radio.value === "auto") {
        root.removeAttribute("data-theme");
        for (const meta of metas) meta.content = /dark/.test(meta.media) ? radio.dataset.dark : radio.dataset.color;
      } else {
        root.setAttribute("data-theme", radio.value);
        for (const meta of metas) meta.content = radio.dataset.color;
      }
    };
    picker.hidden = false;
    apply(root.getAttribute("data-theme") || "auto");
    for (const radio of radios) {
      radio.addEventListener("change", () => {
        if (!radio.checked) return;
        apply(radio.value);
        store.set(THEME_KEY, radio.value);
      });
    }
    // The same choice in the other tabs of the site.
    window.addEventListener("storage", (ev) => {
      if (ev.key === THEME_KEY) apply(ev.newValue || "auto");
    });

    // On a computer the choices open from the button of the bar.
    const setOpen = (open) => {
      toggle.setAttribute("aria-expanded", String(open));
      picker.classList.toggle("is-open", open);
    };
    toggle.addEventListener("click", () => {
      const open = toggle.getAttribute("aria-expanded") !== "true";
      setOpen(open);
      if (open) (radios.find((r) => r.checked) || radios[0]).focus();
    });
    picker.addEventListener("keydown", (ev) => {
      if (!picker.classList.contains("is-open")) return;
      if (ev.key === "Escape" || (ev.key === "Enter" && ev.target.name === "theme")) {
        ev.preventDefault();
        ev.stopPropagation();
        setOpen(false);
        toggle.focus();
      }
    });
    document.addEventListener("click", (ev) => {
      if (!picker.contains(ev.target)) setOpen(false);
    });
    picker.addEventListener("focusout", (ev) => {
      if (ev.relatedTarget && !picker.contains(ev.relatedTarget)) setOpen(false);
    });
    wide.addEventListener("change", () => setOpen(false));
  }
})();
