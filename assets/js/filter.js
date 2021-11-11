var filterActive;

function filterCategory(cat1, cat2, cat3) {
        
    // reset results list
    $('.filter-cat-results .f-cat').removeClass('active');
    
    // the filtering in action for all criteria
    var selector = ".filtering .f-cat";
    if (cat1 !== 'cat-all') {
         selector = '[data-cat=' + cat1 + "]";
    }
    if (cat2 !== 'cat-all') {
        selector = selector + '[data-cat2=' + cat2 + "]";
    }
    if (cat3 !== 'cat-all') {
        selector = selector + '[data-cat3=' + cat3 + "]";
    }
    
    // show all results
    $(selector).addClass('active');

    // reset active filter
    filterActive = cat1;
}

// start by showing all items
$('.filtering .f-cat').addClass('active');

// call the filtering function when selects are changed
$('.filtering select').change(function() {
    
    filterCategory($('.filtering select.cat1').val(), $('.filtering select.cat2').val(), $('.filtering select.cat3').val());
    
});