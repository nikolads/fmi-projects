#include "model.h"
#include <cstdio>

void jsonify_result(const sir::SimulResult& res) {
    printf("sir_result = [\n");
    for (int i = 0; i < res.size(); i++) {
        printf("\t{s: %lf, i: %lf, r: %lf, time: %lf},\n", res[i].S, res[i].I, res[i].R, res[i].time);
    }
    printf("]\n");
}

int main() {
    double infect_prob = 2.18e-3;
    double recovery_rate = 202 * 2.18e-3;
    sir::DataPoint initial_cond = sir::DataPoint(762, 1, 0);

    sir::Model model = sir::Model(initial_cond, infect_prob, recovery_rate);
    sir::SimulResult res = model.simulate(20);
    jsonify_result(res);
}
