// The Rythme website: the burger menu, the copy buttons, the remembered language, and
// the installable app (the service worker, the "new version" banner, the install
// button and the hint for iOS). Every page works without this file: the links then
// stay visible under the logo, and the site is a plain website.
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
  // background and waits: the banner offers to reload, nothing reloads on its own (the
  // playground may be in use).
  const site = document.querySelector(".site[data-sw]");
  const update = $("app-update");
  if (site && update && "serviceWorker" in navigator) {
    let registration = null;
    let reloading = false;
    const offer = () => { update.hidden = false; };
    const watch = (worker) => {
      worker.addEventListener("statechange", () => {
        if (worker.state === "installed" && navigator.serviceWorker.controller) offer();
      });
    };
    window.addEventListener("load", async () => {
      try {
        registration = await navigator.serviceWorker.register(site.dataset.sw, { scope: site.dataset.scope, updateViaCache: "none" });
      } catch {
        return;
      }
      if (!registration) return;
      if (registration.waiting && navigator.serviceWorker.controller) offer();
      if (registration.installing) watch(registration.installing);
      registration.addEventListener("updatefound", () => watch(registration.installing));
      // An app left open looks for a new version when it comes back.
      document.addEventListener("visibilitychange", () => {
        if (document.visibilityState === "visible") registration.update().catch(() => {});
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
})();
