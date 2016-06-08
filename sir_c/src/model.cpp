#include "model.h"

namespace sir {

DataPoint::DataPoint():
    S(0.0),
    I(0.0),
    R(0.0),
    time(0.0)
{
}

DataPoint::DataPoint(double _S, double _I, double _R, double _time):
    S(_S),
    I(_I),
    R(_R),
    time(_time)
{
}

Model::Model(DataPoint _initial_cond, double _infect_prob, double _recovery_rate):
    initial_cond(_initial_cond),
    infect_prob(_infect_prob),
    recovery_rate(_recovery_rate)
{
    this->result_step = 1.0;
    this->sim_step = 0.001;
}

SimulResult Model::simulate(double sim_time) {
    int nresults = int(ceil(sim_time / result_step));
    SimulResult result;
    result.reserve(nresults);

    result.push_back(this->initial_cond);

    DataPoint prev = this->initial_cond;
    DataPoint curr = DataPoint();

    while (prev.time < sim_time) {
        double s_to_i = this->infect_prob * prev.S * prev.I;
        double i_to_r = this->recovery_rate * prev.I;

        curr.S = prev.S - s_to_i * sim_step;
        curr.I = prev.I + (s_to_i - i_to_r) * sim_step;
        curr.R = prev.R + i_to_r * sim_step;
        curr.time = prev.time + sim_step;

        curr.S = std::max(curr.S, 0.0);
        curr.I = std::max(curr.I, 0.0);

        if (int(curr.time / result_step) > int(prev.time / result_step)) {
            result.push_back(curr);
        }

        prev = curr;
    }

    return result;
}

}
