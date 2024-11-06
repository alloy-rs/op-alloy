// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
var sidebarScrollbox = document.querySelector("#sidebar .sidebar-scrollbox");
sidebarScrollbox.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="intro.html"><strong aria-hidden="true">1.</strong> Introduction</a></li><li class="chapter-item expanded "><a href="starting/installation.html"><strong aria-hidden="true">2.</strong> Getting Started</a></li><li class="chapter-item expanded "><a href="building/index.html"><strong aria-hidden="true">3.</strong> Building</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="building/genesis/index.html"><strong aria-hidden="true">3.1.</strong> Genesis</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="building/genesis/rollup-config.html"><strong aria-hidden="true">3.1.1.</strong> Rollup Config</a></li><li class="chapter-item expanded "><a href="building/genesis/system-config.html"><strong aria-hidden="true">3.1.2.</strong> System Config</a></li></ol></li><li class="chapter-item expanded "><a href="building/consensus.html"><strong aria-hidden="true">3.2.</strong> Consensus</a></li><li class="chapter-item expanded "><a href="building/engine.html"><strong aria-hidden="true">3.3.</strong> Engine RPC Types</a></li><li class="chapter-item expanded "><a href="building/protocol/index.html"><strong aria-hidden="true">3.4.</strong> Protocol</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="building/protocol/block-info.html"><strong aria-hidden="true">3.4.1.</strong> BlockInfo and L2BlockInfo Types</a></li></ol></li></ol></li><li class="chapter-item expanded "><a href="examples/index.html"><strong aria-hidden="true">4.</strong> Examples</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="examples/load-a-rollup-config.html"><strong aria-hidden="true">4.1.</strong> Load a Rollup Config</a></li></ol></li><li class="chapter-item expanded "><a href="CONTRIBUTING.html"><strong aria-hidden="true">5.</strong> Contributing</a></li><li class="chapter-item expanded "><a href="LICENSE.html"><strong aria-hidden="true">6.</strong> Licensing</a></li><li class="chapter-item expanded "><a href="glossary.html"><strong aria-hidden="true">7.</strong> Glossary</a></li></ol>';
(function() {
    let current_page = document.location.href.toString();
    if (current_page.endsWith("/")) {
        current_page += "index.html";
    }
    var links = sidebarScrollbox.querySelectorAll("a");
    var l = links.length;
    for (var i = 0; i < l; ++i) {
        var link = links[i];
        var href = link.getAttribute("href");
        if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
            link.href = path_to_root + href;
        }
        // The "index" page is supposed to alias the first chapter in the book.
        if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
            link.classList.add("active");
            var parent = link.parentElement;
            while (parent) {
                if (parent.tagName === "LI" && parent.previousElementSibling) {
                    if (parent.previousElementSibling.classList.contains("chapter-item")) {
                        parent.previousElementSibling.classList.add("expanded");
                    }
                }
                parent = parent.parentElement;
            }
        }
    }
})();

// Track and set sidebar scroll position
sidebarScrollbox.addEventListener('click', function(e) {
    if (e.target.tagName === 'A') {
        sessionStorage.setItem('sidebar-scroll', sidebarScrollbox.scrollTop);
    }
}, { passive: true });
var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
sessionStorage.removeItem('sidebar-scroll');
if (sidebarScrollTop) {
    // preserve sidebar scroll position when navigating via links within sidebar
    sidebarScrollbox.scrollTop = sidebarScrollTop;
} else {
    // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
    var activeSection = document.querySelector('#sidebar .active');
    if (activeSection) {
        activeSection.scrollIntoView({ block: 'center' });
    }
}
