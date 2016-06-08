#include "model.h"
#include "fit_grad_desc.h"
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

void jsonify_result(const char* var_name, const sir::SimulResult& res) {
    printf("%s = [\n", var_name);
    for (int i = 0; i < res.size(); i++) {
        printf("\t{s: %lf, i: %lf, r: %lf, time: %.2lf},\n", res[i].S, res[i].I, res[i].R, res[i].time);
    }
    printf("]\n");
}

void fit();
void model_0();
void model_1();

int main() {
    // fit();
    model_0();
}

// Gradient descend fitting of alpha and beta
void fit() {
    double alpha = 0.055939648467;
    double beta = 0.000840339158;

    auto gd = sir::GradientDesc(target, 119);

    std::pair<double, double> fit = gd.fit(beta, alpha, 1e-13L);
    fprintf(stderr, "%.10lf %.10lf\n", fit.first, fit.second);
}

// Simulation of the fitting results and export in JSON format
void model_0() {
    double fit_beta = 0.000840339158;
    double fit_alpha = 0.055939648467;

    auto model = sir::Model(target[0], fit_beta, fit_alpha);
    auto res = model.simulate(119);

    jsonify_result("target_result", target);
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
