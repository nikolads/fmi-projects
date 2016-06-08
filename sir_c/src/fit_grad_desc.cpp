#include "fit_grad_desc.h"

namespace sir {

// 2.0 ** -34 = 0.0000000000582076609134674072265625
const double EPSILON = 0x1p-34;

GradientDesc::GradientDesc(const SimulResult& _target, double _sim_time) :
    target(_target),
    sim_time(_sim_time),
    precision(1e-15)
{
}

double GradientDesc::error(const SimulResult& curr_res) const {
    int len = std::min(this->target.size(), curr_res.size());
    double err = 0.0;

    for (int i = 0; i < len; i++) {
        double delta = this->target[i].S - curr_res[i].S;
        err += delta * delta;

        delta = this->target[i].I - curr_res[i].I;
        err += delta * delta;
    }

    return err;
}

double GradientDesc::cost(double beta, double alpha) const {
    Model model = Model(this->target[0], beta, alpha);
    SimulResult curr_res = model.simulate(this->sim_time);
    return this->error(curr_res);
}

std::pair<double, double> GradientDesc::cost_derivative(double beta, double alpha) const {
    return std::make_pair(
        (this->cost(beta + EPSILON, alpha) - this->cost(beta, alpha)) / EPSILON,
        (this->cost(beta, alpha + EPSILON) - this->cost(beta, alpha)) / EPSILON
    );
}

std::pair<double, double> GradientDesc::fit(double init_beta, double init_alpha, double init_step) {
    double alpha = init_alpha;
    double beta = init_beta;
    double step = init_step;

    double prev_cost = this->cost(beta, alpha);
    auto transl = std::make_pair(
        std::numeric_limits<double>::infinity(),
        std::numeric_limits<double>::infinity()
    );

    int simul_step = 0;
    int step_reductions = 0;

    while (std::abs(transl.first + transl.second) > this->precision) {
        double curr_cost;

        transl = this->cost_derivative(beta, alpha);
        transl.first *= -step;
        transl.second *= -step;
        curr_cost = this->cost(beta + transl.first, alpha + transl.second);

        while (std::abs(curr_cost) >= std::abs(prev_cost)) {
            step *= 0.5;
            step_reductions++;

            transl = this->cost_derivative(beta, alpha);
            transl.first *= -step;
            transl.second *= -step;
            curr_cost = this->cost(beta + transl.first, alpha + transl.second);
        }

        beta = beta + transl.first;
        alpha = alpha + transl.second;
        step = init_step;

        prev_cost = curr_cost;
        simul_step++;

        if (simul_step % 500 == 0) {
            printf("Simulating %.1fK... ", float(simul_step)/1000);
            printf("beta=%.12lf, alpha=%.12lf, ", beta, alpha);
            printf("cost=%lf, ", curr_cost);
            printf("step_reductions=%d", step_reductions);
            printf("\n");

            step_reductions = 0;
        }
    }

    return std::make_pair(beta, alpha);
}

}
