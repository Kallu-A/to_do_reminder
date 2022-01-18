// allow to easily test if the form is fill or not if not show the message to the error
function isFill(input_el, error, message) {
    let input_password = $(input_el);
    let error_password = $(error);
    let input = input_password.val();
    switch (input) {
        case "" :
            input_password.parent().removeClass().addClass("form_err");
            error_password.text(message);
            break;

        default:
            input_password.parent().removeClass().addClass("form_ok");
            error_password.text("");
    }
}

// use to set an form error with a message from the server
function set_err(name, name_text, message) {
    $(name).parent().removeClass().addClass("form_err");
    $(name_text).text(message)
}

function isMatch(passFirst, passSecond, errorDisplay) {
    if ( passFirst.val() !==  passSecond.val() ) {
        passSecond.parent().removeClass().addClass("form_err");
        errorDisplay.text("doesn't match the password");
    } else {
        if ( passFirst.val() !== "" && passSecond.val() !== "") {
            passSecond.parent().removeClass().addClass("form_ok");
            errorDisplay.text("");
        }
    }
}

function isEmail(email, error_email) {
    const mailformat = /^w+[+.w-]*@([w-]+.)*w+[w-]*.([a-z]{2,4}|d+)$/i;
    if (!email.val().match(mailformat)) {
        console.log("invalide email");
        email.parent().removeClass().addClass("form_err");
        error_email.text("invalid email");
    } else {
        console.log("valide email");
        email.parent().removeClass().addClass("form_ok");
        error_email.text("");
    }
}