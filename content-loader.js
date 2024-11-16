async function loadContent() {
    try {
        const response = await fetch('content.json');
        return await response.json();
    } catch (error) {
        console.error('Error loading content:', error);
        return { articles: [] };
    }
}

window.get_content = function() {
    return window.__CONTENT || { articles: [] };
};

// Load content when the script loads
loadContent().then(content => {
    window.__CONTENT = content;
    // Dispatch an event to notify that content is loaded
    window.dispatchEvent(new CustomEvent('contentLoaded'));
});