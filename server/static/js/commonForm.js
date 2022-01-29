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

/// Handle the input range
function handle_selector() {
    const selector = document.getElementById("input_priority");
    const disp_selector = $('#display_priority');
    switch (selector.value) {
        case "0":
            disp_selector.text('minimum');
            disp_selector.removeClass().addClass("min");
            break;
        case "1" || "2" || "3":
            disp_selector.text("low");
            disp_selector.removeClass().addClass("low");
            break;
        case "4" || "5" || "6":
            disp_selector.text("medium");
            disp_selector.removeClass().addClass("medium");
            break;
        case "7" || "8" || "9":
            disp_selector.text("high");
            disp_selector.removeClass().addClass("high");
            break;
        case "10":
            disp_selector.text("maximum");
            disp_selector.removeClass().addClass("max");
            break;
    }
}

function isEmail(email, error_email) {
    const mailformat = /^\w+([\.-]?\w+)*@\w+([\.-]?\w+)*(\.\w{1,4})+$/;
    if (!email.val().match(mailformat)) {
        email.parent().removeClass().addClass("form_err");
        error_email.text("invalid email");
    } else {
        email.parent().removeClass().addClass("form_ok");
        error_email.text("");
    }
}