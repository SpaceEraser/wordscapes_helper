import("./pkg").then(module => {
    module.init();
    
    window.helper = new module.WordSearcherWrapper();

    $("#given_letters, #filter").prop("disabled", false);

    $("#given_letters").on("keyup", function() {
        $("#filter").val('');
        $("#results").html(window.helper.lookup($(this).val()));
    });

    $("#filter").on("keyup", function() {
        $("#results").html(window.helper.lookup_filter($("#given_letters").val(), $(this).val()));
    });
});
