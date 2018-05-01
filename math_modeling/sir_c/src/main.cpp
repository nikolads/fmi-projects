#include "data.h"
#include "fit/grad_desc.h"
#include "fit/simul_annealing.h"
#include "sir/sir.h"
#include <cstdio>

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

void fit_gd(const sir::SimulResult& target, double beta, double alpha);
void fit_sa(const sir::SimulResult& target, double beta, double alpha);
void model(const sir::SimulResult& target, double beta, double alpha);
void model_basic();

int main() {
    // double beta = 0.001247361578801;
    // double alpha = 0.090635244631641;
    // fit_gd(target_m3, beta, alpha);
    // fit_sa(target_m3, beta, alpha);
    // model(target_m3, beta, alpha);


    double beta = 0.000836677721689;
    double alpha = 0.053104758573852;
    // fit_gd(target_m5, beta, alpha);
    fit_sa(target_m5, beta, alpha);
    // model(target_m5, beta, alpha);
}

// Gradient descend fitting of alpha and beta
void fit_gd(const sir::SimulResult& target, double beta, double alpha) {
    auto gd = GradientDesc(target, target.points.size());

    std::pair<double, double> fit = gd.fit(beta, alpha, 1e-13L);
    fprintf(stderr, "%.15lf %.15lf\n", fit.first, fit.second);
}

// Simulated annealing fitting of alpha and beta
void fit_sa(const sir::SimulResult& target, double beta, double alpha) {
    srand(0);

    auto sa = SimulAnnealing(target, target.points.size());
    sa.init_temp = 10000;
    sa.cooling_rate = 10;
    sa.start(beta, alpha, 0.001, 0.01);
}

// Simulation of the fitting results and export in JSON format
void model(const sir::SimulResult& target, double fit_beta, double fit_alpha) {
    auto model = sir::Model(target.points[0], fit_beta, fit_alpha);
    auto res = model.simulate(target.points.size());

    jsonify_result("target_result", target);
    jsonify_result("sir_result", res);
}

// Plain SIR simulation
void model_basic() {
    double infect_prob = 2.18e-3;
    double recovery_rate = 202 * 2.18e-3;
    sir::DataPoint initial_cond = sir::DataPoint(762, 1, 0);

    sir::Model model = sir::Model(initial_cond, infect_prob, recovery_rate);
    model.result_step = 0.1;

    sir::SimulResult res = model.simulate(20);
    jsonify_result("sir_result", res);
}
