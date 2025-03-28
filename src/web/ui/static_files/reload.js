(function () {
    // reload the page every 5 minutes so that we stay fresh
    // assume that cache-control is working and we are not getting
    // stale content at load time
    setTimeout(function () {
        location.reload();
    }, 5 * 60 * 1000);
})();
