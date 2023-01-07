function openNewTab(url) {
    const newTab = window.open(url, '_self');
}

window.onfocus = () => {
    // A new tab has been created, so run the openNewTab() function
    openNewTab();
}

openNewTab('https://app.skiff.com')