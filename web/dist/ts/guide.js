const channel = new BroadcastChannel("seekr-channel");
import { getValue, getDropdown } from "./dropdown.js";
// Listen for messages on the broadcast channel
channel.addEventListener('message', (event) => {
    if (event.data.type === "theme") {
        const theme = event.data.theme;
        document.documentElement.setAttribute("data-theme", theme);
    }
});
// The actual stuff
const countryDropdown = getDropdown("country");
const checkboxName = document.getElementById("checkbox_01");
const checkboxNameIcon = document.getElementById("checkbox_01_icon");
const checkboxAddress = document.getElementById("checkbox_02");
const checkboxAddressIcon = document.getElementById("checkbox_02_icon");
const checkboxPhone = document.getElementById("checkbox_03");
const checkboxPhoneIcon = document.getElementById("checkbox_03_icon");
const checkboxVIN = document.getElementById("checkbox_04");
const checkboxVINIcon = document.getElementById("checkbox_04_icon");
const checkboxBusiness = document.getElementById("checkbox_05");
const checkboxBusinessIcon = document.getElementById("checkbox_05_icon");
const checkboxIP = document.getElementById("checkbox_06");
const checkboxIPIcon = document.getElementById("checkbox_06_icon");
const checkboxUsername = document.getElementById("checkbox_07");
const checkboxUsernameIcon = document.getElementById("checkbox_07_icon");
const checkboxDomain = document.getElementById("checkbox_08");
const checkboxDomainIcon = document.getElementById("checkbox_08_icon");
const list_elements = document.querySelectorAll(".link-list-holder li");
function resetAll() {
    for (let i = 0; i < list_elements.length; i++) {
        const element = list_elements[i];
        element.style.display = "flex";
    }
}
function checkChecboxValue(checkboxType) {
    if (checkboxType == "name") {
        return checkboxName.checked;
    }
    else if (checkboxType == "address") {
        return checkboxAddress.checked;
    }
    else if (checkboxType == "phone") {
        return checkboxPhone.checked;
    }
    else if (checkboxType == "vin") {
        return checkboxVIN.checked;
    }
    else if (checkboxType == "business") {
        return checkboxBusiness.checked;
    }
    else if (checkboxType == "ip") {
        return checkboxIP.checked;
    }
    else if (checkboxType == "username") {
        return checkboxUsername.checked;
    }
    else if (checkboxType == "domain") {
        return checkboxDomain.checked;
    }
    else {
        return false;
    }
}
function checkCountry() {
    if (document) {
        const selectedCountry = getValue("country");
        if (selectedCountry) {
            const countries = {};
            // English
            countries["Select country:"] = "all";
            countries["WorldWide"] = "ww";
            countries["United States"] = "us";
            countries["Canada"] = "ca";
            countries["United Kingdom"] = "uk";
            countries["Sweden"] = "se";
            countries["Germany"] = "de";
            return countries[selectedCountry]; // Error here
        }
    }
}
function listHandler() {
    let listOfClasses = ["country", "name", "address", "phone", "vin", "business", "ip", "username", "domain"];
    resetAll();
    const selectedCountry = checkCountry();
    // Replace the first element with the country code
    if (selectedCountry != undefined) {
        listOfClasses[0] = selectedCountry;
    }
    if (checkChecboxValue("name") == false) {
        listOfClasses[1] = false;
        checkboxNameIcon.style.opacity = "0";
    }
    else {
        checkboxNameIcon.style.opacity = "1";
    }
    if (checkChecboxValue("address") == false) {
        listOfClasses[2] = false;
        checkboxAddressIcon.style.opacity = "0";
    }
    else {
        checkboxAddressIcon.style.opacity = "1";
    }
    if (checkChecboxValue("phone") == false) {
        listOfClasses[3] = false;
        checkboxPhoneIcon.style.opacity = "0";
    }
    else {
        checkboxPhoneIcon.style.opacity = "1";
    }
    if (checkChecboxValue("vin") == false) {
        listOfClasses[4] = false;
        checkboxVINIcon.style.opacity = "0";
    }
    else {
        checkboxVINIcon.style.opacity = "1";
    }
    if (checkChecboxValue("business") == false) {
        listOfClasses[5] = false;
        checkboxBusinessIcon.style.opacity = "0";
    }
    else {
        checkboxBusinessIcon.style.opacity = "1";
    }
    if (checkChecboxValue("ip") == false) {
        listOfClasses[6] = false;
        checkboxIPIcon.style.opacity = "0";
    }
    else {
        checkboxIPIcon.style.opacity = "1";
    }
    if (checkChecboxValue("username") == false) {
        listOfClasses[7] = false;
        checkboxUsernameIcon.style.opacity = "0";
    }
    else {
        checkboxUsernameIcon.style.opacity = "1";
    }
    if (checkChecboxValue("domain") == false) {
        listOfClasses[8] = false;
        checkboxDomainIcon.style.opacity = "0";
    }
    else {
        checkboxDomainIcon.style.opacity = "1";
    }
    if (listOfClasses[1] == false && listOfClasses[2] == false && listOfClasses[3] == false && listOfClasses[4] == false && listOfClasses[5] == false && listOfClasses[6] == false && listOfClasses[7] == false && listOfClasses[8] == false) {
        for (let i = 0; i < list_elements.length; i++) {
            const element = list_elements[i];
            if (listOfClasses[0] == "all") {
                element.style.display = "flex";
            }
            else if (!element.classList.contains(listOfClasses[0].toString())) {
                element.style.display = "none";
            }
        }
    }
    else {
        for (let i = 0; i < list_elements.length; i++) {
            const element = list_elements[i];
            if (!element.classList.contains(listOfClasses[0].toString()) && listOfClasses[0] != "all") {
                element.style.display = "none";
            }
            else {
                let hasBeenChanged = false;
                for (let i = 1; i < listOfClasses.length; i++) {
                    if (element.classList.contains(listOfClasses[i].toString())) {
                        element.style.display = "flex";
                        hasBeenChanged = true;
                    }
                    if (i == 8 && hasBeenChanged == false) {
                        element.style.display = "none";
                    }
                }
            }
        }
    }
}
function preListHandler() {
    listHandler();
}
checkboxName.addEventListener('change', preListHandler);
checkboxAddress.addEventListener('change', preListHandler);
checkboxPhone.addEventListener('change', preListHandler);
checkboxVIN.addEventListener('change', preListHandler);
checkboxBusiness.addEventListener('change', preListHandler);
checkboxIP.addEventListener('change', preListHandler);
checkboxUsername.addEventListener('change', preListHandler);
checkboxDomain.addEventListener('change', preListHandler);
countryDropdown.addEventListener('change', preListHandler);
export { listHandler };
