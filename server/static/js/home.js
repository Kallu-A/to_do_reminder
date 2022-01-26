$(window).on('resize', function(){
    var win = $(this); //this = window
    if (win.width() <= 600) {
        $("#log").height($("#not_log").height());
        $("#log").width($("#not_log").width());
    }
});