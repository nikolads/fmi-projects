var myLineChart;
var sir_result = sir_result || [];

jQuery(function(){
    var data = {
        labels: [],
        datasets: [{
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

    Chart.defaults.global.defaultFontColor = "#fff";

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

    sir_result.forEach(function(item) {
        myLineChart.data.labels.push(item.time);
        myLineChart.data.datasets[0].data.push(item.s);
        myLineChart.data.datasets[1].data.push(item.i);
        myLineChart.data.datasets[2].data.push(item.r);
        myLineChart.update();
    });
});
