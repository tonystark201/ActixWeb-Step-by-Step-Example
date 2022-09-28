/*
Deal with Teacher:
    renderTeacher
    teacherApiCall
    editTeacher
    deleteTeacher
    getTeachers
    createTeacher
*/
function renderTeacher(items,elementId) {
    let placeholder = "<div>"
    let itemsMeta = [];

    for (i = 0; i < items.length; i++) {
        let uid = items[i]["uid"].toString();
        let name = items[i]["name"]
        let age = items[i]["age"].toString();

        let editId = "edit" + "-" + "teacher"+ "-"+ uid.replaceAll(" ", "-");
        let deleteId = "delete" + "-" + "teacher"+ "-" + uid.replaceAll(" ", "-");

        placeholder += "<div>" + "id:"+ uid + ";" + "name:"+name + ";" + "age:"+ age + ";" +
            "<button " + 'id="' + editId + '">'
            + "edit" +
            '</button>' +
            "<button " + 'id="' + deleteId + '">'
            + "delete" +
            '</button>' +
            "</div>";
        itemsMeta.push({"edit_id": editId, "delete_id":deleteId, "uid": uid,"name":name,"age":age});
    }
    placeholder += "</div>"
    console.log(placeholder);
    document.getElementById(elementId).innerHTML = placeholder;

    for (i = 0; i < itemsMeta.length; i++) {
        document.getElementById(itemsMeta[i]["edit_id"]).addEventListener( "click", editTeacher);
        document.getElementById(itemsMeta[i]["delete_id"]).addEventListener( "click", deleteTeacher);
    }
}

function teacherApiCall(url, method) {
    let xhr = new XMLHttpRequest();
    xhr.withCredentials = true;
    xhr.addEventListener('readystatechange', function() {
        if (this.readyState === this.DONE) {
            renderTeacher(JSON.parse(this.responseText),"teacher");
        }
    });
    xhr.open(method, url);
    xhr.setRequestHeader('content-type', 'application/json');
    xhr.setRequestHeader('user-token', 'token');
    return xhr
}

function editTeacher() {
    let uid = this.id.replaceAll("-", " ").replace("edit teacher ", "");
    let url = "/v1/teacher/"+uid;
    let call = teacherApiCall(url, "PUT");
    let json = {
        "name": "JamesModify",
        "age": 20
    };
    call.send(JSON.stringify(json));
    setTimeout(getTeachers, 200);
}

function deleteTeacher() {
    let uid = this.id.replaceAll("-", " ").replace("delete teacher ", "");
    let url = "/v1/teacher/"+uid;
    console.log(url);
    let call = teacherApiCall(url, "DELETE");
    call.send();
    setTimeout(getTeachers, 200);
}

function getTeachers() {
    let url = "/v1/teachers";
    let call = teacherApiCall(url, 'GET');
    call.send()
}
getTeachers();

document.getElementById("tsubmit").addEventListener("click", createTeacher);

function createTeacher() {
    let url = "/v1/teachers";
    let name = document.getElementById("tname");
    let age = document.getElementById("tage");
    let call = teacherApiCall(url, "POST");
    let json = {
         "name": name.value,
         "age": parseInt(age.value, 10)
    };
    console.log(json);
    call.send(JSON.stringify(json));
    document.getElementById("tname").value = null;
    document.getElementById("tage").value = null;
    setTimeout(getTeachers,200);
}



/*
Deal with Student:
    Student
    studentApiCall
    editStudent
    deleteStudent
    getStudents
    createStudent
*/
function renderStudent(items,elementId) {
    let placeholder = "<div>"
    let itemsMeta = [];

    for (i = 0; i < items.length; i++) {
        let uid = items[i]["uid"].toString();
        let name = items[i]["name"]
        let age = items[i]["age"].toString();

        let editId = "edit" + "-" + "student"+ "-"+ uid.replaceAll(" ", "-");
        let deleteId = "delete" + "-" + "student"+ "-" + uid.replaceAll(" ", "-");

        placeholder += "<div>" + "id:"+ uid + ";" + "name:"+name + ";" + "age:"+ age + ";" +
            "<button " + 'id="' + editId + '">'
            + "edit" +
            '</button>' +
            "<button " + 'id="' + deleteId + '">'
            + "delete" +
            '</button>' +
            "</div>";
        itemsMeta.push({"edit_id": editId, "delete_id":deleteId, "uid": uid,"name":name,"age":age});
    }
    placeholder += "</div>"
    console.log(placeholder);
    document.getElementById(elementId).innerHTML = placeholder;

    for (i = 0; i < itemsMeta.length; i++) {
        document.getElementById(itemsMeta[i]["edit_id"]).addEventListener( "click", editStudent);
        document.getElementById(itemsMeta[i]["delete_id"]).addEventListener( "click", deleteStudent);
    }
}

function studentApiCall(url, method) {
    let xhr = new XMLHttpRequest();
    xhr.withCredentials = true;
    xhr.addEventListener('readystatechange', function() {
        if (this.readyState === this.DONE) {
            renderStudent(JSON.parse(this.responseText),"student");
        }
    });
    xhr.open(method, url);
    xhr.setRequestHeader('content-type', 'application/json');
    xhr.setRequestHeader('user-token', 'token');
    return xhr
}

function editStudent() {
    let uid = this.id.replaceAll("-", " ").replace("edit student ", "");
    let url = "/v1/student/"+uid;
    let call = studentApiCall(url, "PUT");
    let json = {
        "name": "AliceModiy",
        "age": 30
    };
    call.send(JSON.stringify(json));
    setTimeout(getStudents, 200);
}

function deleteStudent() {
    let uid = this.id.replaceAll("-", " ").replace("delete student ", "");
    let url = "/v1/student/"+uid;
    console.log(url);
    let call = studentApiCall(url, "DELETE");
    call.send();
    setTimeout(getStudents, 200);
}

function getStudents() {
    let url = "/v1/students";
    let call = studentApiCall(url, 'GET');
    call.send()
}
getStudents();

document.getElementById("ssubmit").addEventListener("click", createStudent);

function createStudent() {
    let url = "/v1/students";
    let name = document.getElementById("sname");
    let age = document.getElementById("sage");
    let call = studentApiCall(url, "POST");
    let json = {
         "name": name.value,
         "age": parseInt(age.value, 10)
    };
    console.log(json);
    call.send(JSON.stringify(json));
    document.getElementById("sname").value = null;
    document.getElementById("sage").value = null;
    setTimeout(getStudents,200);
}
