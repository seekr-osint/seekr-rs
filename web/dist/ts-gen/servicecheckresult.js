/* Do not change, this code is generated from Golang structs */
export class Errors {
    exists;
    info;
    constructor(source = {}) {
        if ('string' === typeof source)
            source = JSON.parse(source);
        this.exists = source["exists"];
        this.info = source["info"];
    }
}
export class AccountInfo {
    url;
    profile_picture;
    bio;
    constructor(source = {}) {
        if ('string' === typeof source)
            source = JSON.parse(source);
        this.url = source["url"];
        this.profile_picture = source["profile_picture"];
        this.bio = source["bio"];
    }
}
export class Service {
    name;
    domain;
    constructor(source = {}) {
        if ('string' === typeof source)
            source = JSON.parse(source);
        this.name = source["name"];
        this.domain = source["domain"];
    }
}
export class User {
    Username;
    constructor(source = {}) {
        if ('string' === typeof source)
            source = JSON.parse(source);
        this.Username = source["Username"];
    }
}
export class InputData {
    user;
    service;
    constructor(source = {}) {
        if ('string' === typeof source)
            source = JSON.parse(source);
        this.user = this.convertValues(source["user"], User);
        this.service = this.convertValues(source["service"], Service);
    }
    convertValues(a, classs, asMap = false) {
        if (!a) {
            return a;
        }
        if (a.slice) {
            return a.map(elem => this.convertValues(elem, classs));
        }
        else if ("object" === typeof a) {
            if (asMap) {
                for (const key of Object.keys(a)) {
                    a[key] = new classs(a[key]);
                }
                return a;
            }
            return new classs(a);
        }
        return a;
    }
}
export class ServiceCheckResult {
    input_data;
    exists;
    info;
    errors;
    constructor(source = {}) {
        if ('string' === typeof source)
            source = JSON.parse(source);
        this.input_data = this.convertValues(source["input_data"], InputData);
        this.exists = source["exists"];
        this.info = this.convertValues(source["info"], AccountInfo);
        this.errors = this.convertValues(source["errors"], Errors);
    }
    convertValues(a, classs, asMap = false) {
        if (!a) {
            return a;
        }
        if (a.slice) {
            return a.map(elem => this.convertValues(elem, classs));
        }
        else if ("object" === typeof a) {
            if (asMap) {
                for (const key of Object.keys(a)) {
                    a[key] = new classs(a[key]);
                }
                return a;
            }
            return new classs(a);
        }
        return a;
    }
}
