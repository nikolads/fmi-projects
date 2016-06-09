#include "simul_annealing.h"

double float_rand() {
    return double(rand()) / double(RAND_MAX);
}

struct OptimalSol {
    std::mutex mtx;
    std::pair<double, double> sol;
    double cost;
};

class Simulation {
public:
    Simulation(const sir::SimulResult& _target, double _sim_time, double init_temp, double _cooling_rate, OptimalSol& opt);

    void start(double init_beta, double init_alpha, double max_step);

private:
    double cost(double beta, double alpha) const;
    double acceptance(double new_cost) const;

private:
    const sir::SimulResult& target;
    double sim_time;

    double temp;
    double init_temp;
    double cooling_rate;

    std::pair<double, double> curr;
    double curr_cost;

    std::pair<double, double> optimal;
    double optimal_cost;

    OptimalSol& global_optimal;
};

Simulation::Simulation(const sir::SimulResult& _target, double _sim_time, double _init_temp, double _cooling_rate, OptimalSol& opt):
    target(_target),
    sim_time(_sim_time),
    init_temp(_init_temp),
    cooling_rate(_cooling_rate),
    global_optimal(opt)
{
}

void Simulation::start(double init_beta, double init_alpha, double max_step) {
    this->curr = std::make_pair(init_beta, init_alpha);
    this->curr_cost = this->cost(this->curr.first, this->curr.second);

    this->optimal = this->curr;
    this->optimal_cost = this->curr_cost;

    while (true) {
        this->temp = this->init_temp;

        while (this->temp != 0) {
            double angle = 360 * float_rand();
            double dist = max_step * std::max(0.1, this->temp / this->init_temp) * float_rand();

            double new_beta = this->curr.first + cos(angle) * dist;
            double new_alpha = this->curr.second + sin(angle) * dist;
            double new_cost = this->cost(new_beta, new_alpha);

            if (this->acceptance(new_cost) > float_rand()) {
                this->curr = std::make_pair(new_beta, new_alpha);
                this->curr_cost = new_cost;

                if (this->curr_cost < this->optimal_cost) {
                    this->optimal = this->curr;
                    this->optimal_cost = this->curr_cost;
                }
            }

            this->temp -= this->cooling_rate;
        }

        this->global_optimal.mtx.lock();
        if (this->optimal_cost < this->global_optimal.cost) {
            this->global_optimal.sol = this->optimal;
            this->global_optimal.cost = this->optimal_cost;

            printf("optimal: (%.15lf, %.15lf), cost=%lf\n", optimal.first, optimal.second, optimal_cost);
        }
        this->global_optimal.mtx.unlock();
    }
}

double Simulation::cost(double beta, double alpha) const {
    sir::Model model = sir::Model(this->target.points[0], beta, alpha);
    sir::SimulResult res = model.simulate(this->sim_time);
    return this->target.error(res);
}

double Simulation::acceptance(double new_cost) const {
    return exp((new_cost - this->curr_cost) / this->temp);
}


SimulAnnealing::SimulAnnealing(const sir::SimulResult& _target, double _sim_time):
    nthreads(4),
    init_temp(1000),
    cooling_rate(1),
    target(_target),
    sim_time(_sim_time)
{
}

void SimulAnnealing::start(double init_beta, double init_alpha, double max_step) {
    OptimalSol opt;
    opt.cost = std::numeric_limits<double>::infinity();

    std::vector<std::thread> threads;

    for (int i = 0; i < this->nthreads; i++) {
        Simulation simul = Simulation(this->target, this->sim_time, this->init_temp, this->cooling_rate, opt);
        threads.push_back(std::thread(&Simulation::start, std::move(simul), init_beta, init_alpha, max_step));
    }

    for (int i = 0; i < this->nthreads; i++) {
        threads[i].join();
    }
}
