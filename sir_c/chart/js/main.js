var myLineChart;
var sir_result = sir_result || [];
var target_result = target_result || [];

jQuery(function(){
    var data = {
        labels: [],
        datasets: [{
            scaleFontColor: '#fff',
            label: '# susceptible (target)',
            lineTension: 0,
            backgroundColor: "rgba(99, 210, 151, 0.3)",
            // borderColor: "#63D297",
            borderColor: "#fff",
            pointBackgroundColor: "#238257",
            fill: false,
            data: [],
        },
        {
            label: '# infected (target)',
            lineTension: 0,
            backgroundColor: "rgba(244,67,54,0.5)",
            // borderColor: "#F44336",
            borderColor: "#fff",
            pointBackgroundColor: "#B40316",
            fill: false,
            data: [],
        },
        {
            scaleFontColor: '#fff',
            label: '# susceptible',
            lineTension: 0,
            backgroundColor: "rgba(99, 210, 151, 0.3)",
            borderColor: "#63D297",
            pointBackgroundColor: "#63D297",
            data: [],
        },
        {
            label: '# infected',
            lineTension: 0,
            backgroundColor: "rgba(244,67,54,0.5)",
            borderColor: "#F44336",
            pointBackgroundColor: "#F44336",
            data: [],
        },
        {
            label: '# recoverd',
            lineTension: 0,
            backgroundColor: "rgba(255,193,7,0.5)",
            borderColor: "#FFC107",
            pointBackgroundColor: "#FFC107",
            data: [],
        }],
    };

    myLineChart = new Chart($('#sir_graphic'), {
        type: 'line',
        data: data,
        options: {
            legend : {
                labels : {
                    fontSize: 16,
                }
            }
        }
    });


    Chart.defaults.global.defaultFontColor = "#fff";

    sir_result.forEach(function(item) {
        myLineChart.data.labels.push(item.time);
        myLineChart.data.datasets[2].data.push(item.s);
        myLineChart.data.datasets[3].data.push(item.i);
        myLineChart.data.datasets[4].data.push(item.r);
    });

    target_result.forEach(function(item) {
        myLineChart.data.datasets[0].data.push(item.s);
        myLineChart.data.datasets[1].data.push(item.i);
    });

    myLineChart.update();
});
