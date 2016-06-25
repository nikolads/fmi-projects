#include "sir/sir.h"
#include "fit/grad_desc.h"
#include "fit/simul_annealing.h"
#include <cstdio>

sir::SimulResult target = sir::SimulResult({
    sir::DataPoint(264, 1, 0, 1),
    sir::DataPoint(264, 1, 0, 2),
    sir::DataPoint(263, 2, 0, 3),
    sir::DataPoint(263, 2, 0, 4),
    sir::DataPoint(261, 4, 0, 5),
    sir::DataPoint(259, 6, 0, 6),
    sir::DataPoint(257, 8, 0, 7),
    sir::DataPoint(256, 9, 0, 8),
    sir::DataPoint(255, 10, 0, 9),
    sir::DataPoint(254, 11, 0, 10),
    sir::DataPoint(253, 12, 0, 11),
    sir::DataPoint(253, 12, 0, 12),
    sir::DataPoint(251, 14, 0, 13),
    sir::DataPoint(250, 15, 0, 14),
    sir::DataPoint(245, 20, 0, 15),
    sir::DataPoint(243, 22, 0, 16),
    sir::DataPoint(237, 28, 0, 17),
    sir::DataPoint(234, 31, 0, 18),
    sir::DataPoint(228, 37, 0, 19),
    sir::DataPoint(221, 44, 0, 20),
    sir::DataPoint(212, 53, 0, 21),
    sir::DataPoint(207, 58, 0, 22),
    sir::DataPoint(200, 65, 0, 23),
    sir::DataPoint(189, 76, 0, 24),
    sir::DataPoint(186, 78, 1, 25),
    sir::DataPoint(179, 85, 1, 26),
    sir::DataPoint(174, 90, 1, 27),
    sir::DataPoint(169, 94, 2, 28),
    sir::DataPoint(163, 100, 2, 29),
    sir::DataPoint(159, 102, 4, 30),
    sir::DataPoint(154, 105, 6, 31),
    sir::DataPoint(146, 111, 8, 32),
    sir::DataPoint(136, 120, 9, 33),
    sir::DataPoint(131, 124, 10, 34),
    sir::DataPoint(125, 129, 11, 35),
    sir::DataPoint(120, 133, 12, 36),
    sir::DataPoint(113, 140, 12, 37),
    sir::DataPoint(107, 144, 14, 38),
    sir::DataPoint(103, 147, 15, 39),
    sir::DataPoint(96, 149, 20, 40),
    sir::DataPoint(96, 147, 22, 41),
    sir::DataPoint(87, 150, 28, 42),
    sir::DataPoint(83, 151, 31, 43),
    sir::DataPoint(75, 153, 37, 44),
    sir::DataPoint(73, 148, 44, 45),
    sir::DataPoint(69, 143, 53, 46),
    sir::DataPoint(64, 143, 58, 47),
    sir::DataPoint(62, 138, 65, 48),
    sir::DataPoint(60, 129, 76, 49),
    sir::DataPoint(58, 128, 79, 50),
    sir::DataPoint(57, 122, 86, 51),
    sir::DataPoint(55, 119, 91, 52),
    sir::DataPoint(55, 114, 96, 53),
    sir::DataPoint(52, 111, 102, 54),
    sir::DataPoint(52, 107, 106, 55),
    sir::DataPoint(51, 103, 111, 56),
    sir::DataPoint(50, 96, 119, 57),
    sir::DataPoint(50, 86, 129, 58),
    sir::DataPoint(49, 82, 134, 59),
    sir::DataPoint(49, 76, 140, 60),
    sir::DataPoint(48, 72, 145, 61),
    sir::DataPoint(48, 65, 152, 62),
    sir::DataPoint(47, 60, 158, 63),
    sir::DataPoint(47, 56, 162, 64),
    sir::DataPoint(47, 49, 169, 65),
    sir::DataPoint(47, 49, 169, 66),
    sir::DataPoint(47, 40, 178, 67),
    sir::DataPoint(47, 36, 182, 68),
    sir::DataPoint(47, 28, 190, 69),
    sir::DataPoint(47, 26, 192, 70),
    sir::DataPoint(47, 22, 196, 71),
    sir::DataPoint(47, 17, 201, 72),
    sir::DataPoint(47, 15, 203, 73),
    sir::DataPoint(46, 14, 205, 74),
    sir::DataPoint(46, 12, 207, 75),
    sir::DataPoint(46, 11, 208, 76),
    sir::DataPoint(46, 9, 210, 77),
    sir::DataPoint(45, 10, 210, 78),
    sir::DataPoint(44, 8, 213, 79),
    sir::DataPoint(44, 8, 213, 80),
    sir::DataPoint(44, 7, 214, 81),
    sir::DataPoint(44, 6, 215, 82),
    sir::DataPoint(44, 6, 215, 83),
    sir::DataPoint(44, 5, 216, 84),
    sir::DataPoint(44, 5, 216, 85),
    sir::DataPoint(44, 4, 217, 86),
    sir::DataPoint(44, 4, 217, 87),
    sir::DataPoint(43, 4, 218, 88),
    sir::DataPoint(43, 4, 218, 89),
    sir::DataPoint(43, 4, 218, 90),
    sir::DataPoint(43, 4, 218, 91),
    sir::DataPoint(43, 4, 218, 92),
    sir::DataPoint(43, 4, 218, 93),
    sir::DataPoint(42, 5, 218, 94),
    sir::DataPoint(42, 5, 218, 95),
    sir::DataPoint(42, 5, 218, 96),
    sir::DataPoint(42, 5, 218, 97),
    sir::DataPoint(42, 5, 218, 98),
    sir::DataPoint(42, 4, 219, 99),
    sir::DataPoint(42, 4, 219, 100),
    sir::DataPoint(42, 4, 219, 101),
    sir::DataPoint(42, 4, 219, 102),
    sir::DataPoint(42, 3, 220, 103),
    sir::DataPoint(42, 2, 221, 104),
    sir::DataPoint(42, 2, 221, 105),
    sir::DataPoint(42, 2, 221, 106),
    sir::DataPoint(42, 2, 221, 107),
    sir::DataPoint(42, 2, 221, 108),
    sir::DataPoint(42, 2, 221, 109),
    sir::DataPoint(42, 2, 221, 110),
    sir::DataPoint(42, 2, 221, 111),
    sir::DataPoint(42, 2, 221, 112),
    sir::DataPoint(42, 1, 222, 113),
    sir::DataPoint(42, 1, 222, 114),
    sir::DataPoint(42, 1, 222, 115),
    sir::DataPoint(42, 1, 222, 116),
    sir::DataPoint(42, 1, 222, 117),
    sir::DataPoint(42, 1, 222, 118),
    sir::DataPoint(42, 0, 223, 119),
});

sir::SimulResult target_avg = sir::SimulResult({
    sir::DataPoint(263.8, 1.2, 0.0, 1.0),
    sir::DataPoint(263.6, 1.4, 0.0, 2.0),
    sir::DataPoint(263.2, 1.8, 0.0, 3.0),
    sir::DataPoint(262.6, 2.4, 0.0, 4.0),
    sir::DataPoint(261.8, 3.2, 0.0, 5.0),
    sir::DataPoint(261.4, 3.6, 0.0, 6.0),
    sir::DataPoint(260.4, 4.6, 0.0, 7.0),
    sir::DataPoint(259.2, 5.8, 0.0, 8.0),
    sir::DataPoint(257.0, 8.0, 0.0, 9.0),
    sir::DataPoint(254.6, 10.4, 0.0, 10.0),
    sir::DataPoint(251.2, 13.8, 0.0, 11.0),
    sir::DataPoint(248.4, 16.6, 0.0, 12.0),
    sir::DataPoint(243.2, 20.8, 1.0, 13.0),
    sir::DataPoint(236.6, 27.2, 1.2, 14.0),
    sir::DataPoint(229.2, 34.4, 1.4, 15.0),
    sir::DataPoint(221.0, 42.2, 1.8, 16.0),
    sir::DataPoint(211.0, 51.6, 2.4, 17.0),
    sir::DataPoint(199.2, 62.6, 3.2, 18.0),
    sir::DataPoint(189.4, 72.0, 3.6, 19.0),
    sir::DataPoint(177.2, 83.2, 4.6, 20.0),
    sir::DataPoint(164.0, 95.2, 5.8, 21.0),
    sir::DataPoint(150.4, 106.6, 8.0, 22.0),
    sir::DataPoint(134.4, 120.2, 10.4, 23.0),
    sir::DataPoint(123.2, 128.0, 13.8, 24.0),
    sir::DataPoint(110.6, 137.8, 16.6, 25.0),
    sir::DataPoint(100.4, 142.8, 21.8, 26.0),
    sir::DataPoint(88.8, 147.8, 28.4, 27.0),
    sir::DataPoint(82.4, 146.8, 35.8, 28.0),
    sir::DataPoint(75.0, 146.0, 44.0, 29.0),
    sir::DataPoint(69.4, 141.6, 54.0, 30.0),
    sir::DataPoint(65.4, 133.8, 65.8, 31.0),
    sir::DataPoint(60.2, 129.2, 75.6, 32.0),
    sir::DataPoint(57.6, 119.6, 87.8, 33.0),
    sir::DataPoint(54.2, 109.8, 101.0, 34.0),
    sir::DataPoint(52.2, 98.2, 114.6, 35.0),
    sir::DataPoint(49.4, 85.0, 130.6, 36.0),
    sir::DataPoint(48.0, 75.2, 141.8, 37.0),
    sir::DataPoint(47.6, 63.0, 154.4, 38.0),
    sir::DataPoint(45.4, 55.0, 164.6, 39.0),
    sir::DataPoint(44.0, 44.8, 176.2, 40.0),
    sir::DataPoint(42.8, 39.6, 182.6, 41.0),
    sir::DataPoint(42.0, 33.0, 190.0, 42.0),
    sir::DataPoint(41.4, 28.0, 195.6, 43.0),
    sir::DataPoint(40.8, 24.6, 199.6, 44.0),
    sir::DataPoint(40.0, 20.2, 204.8, 45.0),
    sir::DataPoint(39.4, 18.2, 207.4, 46.0),
    sir::DataPoint(38.2, 16.0, 210.8, 47.0),
    sir::DataPoint(38.2, 14.0, 212.8, 48.0),
    sir::DataPoint(38.0, 11.4, 215.6, 49.0),
    sir::DataPoint(37.8, 10.2, 217.0, 50.0),
    sir::DataPoint(37.6, 10.0, 217.4, 51.0),
    sir::DataPoint(37.4, 8.0, 219.6, 52.0),
    sir::DataPoint(37.4, 6.6, 221.0, 53.0),
    sir::DataPoint(37.4, 5.4, 222.2, 54.0),
    sir::DataPoint(37.2, 4.8, 223.0, 55.0),
    sir::DataPoint(37.0, 4.4, 223.6, 56.0),
    sir::DataPoint(37.0, 3.8, 224.2, 57.0),
    sir::DataPoint(36.8, 3.2, 225.0, 58.0),
    sir::DataPoint(36.8, 2.6, 225.6, 59.0),
});

void jsonify_result(const char* var_name, const sir::SimulResult& res) {
    printf("%s = [\n", var_name);
    for (int i = 0; i < res.points.size(); i++) {
        printf("\t{s: %lf, i: %lf, r: %lf, time: %.2lf},\n",
            res.points[i].S,
            res.points[i].I,
            res.points[i].R,
            res.points[i].time
        );
    }
    printf("]\n");
}

void fit0();
void fit1();
void model_0();
void model_1();

int main() {
    // fit0();
    fit1();
    // model_0();
}

// Gradient descend fitting of alpha and beta
void fit0() {
    // double beta = 0.000836677721689;
    // double alpha = 0.053104758573852;

    double beta = 0.001247361578801;
    double alpha = 0.090635244631641;

    // auto gd = GradientDesc(target, 100);
    auto gd = GradientDesc(target_avg, 59);

    std::pair<double, double> fit = gd.fit(beta, alpha, 1e-13L);
    fprintf(stderr, "%.15lf %.15lf\n", fit.first, fit.second);
}

// Simulated annealing fitting of alpha and beta
void fit1() {
    double beta = 0.000836677721689;
    double alpha = 0.053104758573852;

    srand(0);

    // auto sa = SimulAnnealing(target, 70);
    auto sa = SimulAnnealing(target_avg, 59);
    sa.start(beta, alpha, 0.001, 0.01);
}

// Simulation of the fitting results and export in JSON format
void model_0() {
    // double fit_beta = 0.000810617767452;
    // double fit_alpha = 0.044023992163282;

    // auto model = sir::Model(target.points[0], fit_beta, fit_alpha);
    // auto res = model.simulate(119);

    // jsonify_result("target_result", target);
    // jsonify_result("sir_result", res);

    double fit_beta = 0.001247361578801;
    double fit_alpha = 0.090635244631641;

    auto model = sir::Model(target_avg.points[0], fit_beta, fit_alpha);
    auto res = model.simulate(59);

    jsonify_result("target_result", target_avg);
    jsonify_result("sir_result", res);
}

// Plain SIR simulation
void model_1() {
    double infect_prob = 2.18e-3;
    double recovery_rate = 202 * 2.18e-3;
    sir::DataPoint initial_cond = sir::DataPoint(762, 1, 0);

    sir::Model model = sir::Model(initial_cond, infect_prob, recovery_rate);
    model.result_step = 0.1;

    sir::SimulResult res = model.simulate(20);
    jsonify_result("sir_result", res);
}
