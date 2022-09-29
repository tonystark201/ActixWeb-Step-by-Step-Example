if (localStorage.getItem("user-token") == null) {
    window.location.replace(document.location.origin + "/login");
} else {
    // Load teacher and student cached data
    let teacher_cached_date = Date.parse(localStorage.getItem("teacher-cache-date"));
    let student_cached_date = Date.parse(localStorage.getItem("student-cache-date"));
    let now = new Date();
    let diff_teacher = Math.round((now - teacher_cached_date) / (1000));
    let diff_student = Math.round((now - student_cached_date) / (1000));

    console.log((diff_student,diff_teacher))

    if (diff_teacher <= 120) {
        renderTeacher(JSON.parse(localStorage.getItem("teacher-cache-data")),"teacher");
    } else {
        getTeachers();
    }

    if (diff_student <= 120) {
        renderStudent(JSON.parse(localStorage.getItem("student-cache-data")),"student");
    } else {
        getStudents();
    }
}


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
           if (this.status === 401) {
               window.location.replace(document.location.origin + "/login");
           } else {
                renderTeacher(JSON.parse(this.responseText),"teacher");
                localStorage.setItem("teacher-cache-date", new Date());
                localStorage.setItem("teacher-cache-data", this.responseText);
           }
        }
    });
    xhr.open(method, url);
    xhr.setRequestHeader('content-type', 'application/json');
    let token = "BEARER " + localStorage.getItem("user-token")
    xhr.setRequestHeader('AUTHORIZATION', token);
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
    let call = teacherApiCall(url, "DELETE");
    call.send();
    setTimeout(getTeachers, 200);
}

function getTeachers() {
    let url = "/v1/teachers";
    let call = teacherApiCall(url, 'GET');
    call.send()
}

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
            if (this.status === 401) {
                window.location.replace(document.location.origin + "/login");
            } else {
                renderStudent(JSON.parse(this.responseText),"student");
                localStorage.setItem("student-cache-date", new Date());
                localStorage.setItem("student-cache-data", this.responseText);
            }
        }
    });
    xhr.open(method, url);
    xhr.setRequestHeader('content-type', 'application/json');
    let token = "BEARER " + localStorage.getItem("user-token")
    xhr.setRequestHeader('AUTHORIZATION', token);
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
    let call = studentApiCall(url, "DELETE");
    call.send();
    setTimeout(getStudents, 200);
}

function getStudents() {
    let url = "/v1/students";
    let call = studentApiCall(url, 'GET');
    call.send()
}


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
    call.send(JSON.stringify(json));
    document.getElementById("sname").value = null;
    document.getElementById("sage").value = null;
    setTimeout(getStudents,200);
}
