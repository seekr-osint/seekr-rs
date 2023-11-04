function saveAsFile(textContent, fileName) {
    // saveAsFile("text","filename.txt");
    const textEncoding = "text/plain;charset=utf-8";
    try {
        var blob = new Blob([textContent], { type: textEncoding });
        saveAs(blob, fileName);
    }
    catch (exception) {
        window.open("data:" + textEncoding + "," + encodeURIComponent(textContent), '_blank', '');
    }
}
function loadDropdown(dropdownType, data) {
    const scrollbox = document.querySelector("body > .edit-container > div > div.scroll-box");
    const dropdownElement = scrollbox.querySelector("custom-dropdown[title='" + dropdownType + "']").shadowRoot.querySelector("div > .table > .dropdown-select > .select-selected");
    if (data != "") {
        dropdownElement.innerHTML = data;
    }
    else {
        dropdownElement.innerHTML = "Select " + dropdownType + ":";
    }
}
function checkDropdownValue(windowType, dropdownType) {
    const scrollbox = document.querySelector("body > div." + windowType + "-container > div > div.scroll-box");
    const selectedType = scrollbox.querySelector("custom-dropdown[title='" + dropdownType + "']").shadowRoot.querySelector("div > .table > .dropdown-select > .select-selected").innerHTML ?? "";
    if (selectedType == "Select " + dropdownType + ":") {
        return "";
    }
    else {
        return selectedType;
    }
}
function apiCall(endpoint) {
    var hostname = window.location.hostname;
    var port = window.location.port;
    var baseUrl = hostname + ":" + port;
    var apiUrl = "http://" + baseUrl + "/api/v1";
    if (endpoint.charAt(0) === "/") {
        endpoint = endpoint.substring(1);
    }
    return apiUrl + '/' + endpoint;
}
export { saveAsFile, loadDropdown, checkDropdownValue, apiCall };
