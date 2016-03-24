from __future__ import division
import numpy as np
import math
import csv

class SimResult:
    def __init__(self, S, I, R, time, len):
        self.S = S
        self.I = I
        self.R = R
        self.t = time
        self.len = len

# Make a simulation using the SIR model
def simulate(s0, i0, r0, infect_prob, recovery_rate, sim_time, **kwargs):
    precision = kwargs.pop("precision", 1.0)
    sim_step = kwargs.pop("sim_step", 0.01)

    steps = int(math.ceil(sim_time / precision))

    S = np.zeros(steps)
    I = np.zeros(steps)
    R = np.zeros(steps)
    T = np.linspace(0, sim_time, steps)

    S[0] = float(s0)
    I[0] = float(i0)
    R[0] = float(r0)

    def s_to_i(prev_s, prev_i):
        return infect_prob * prev_s * prev_i

    def i_to_r(prev_i):
        return recovery_rate * prev_i

    prev_s = S[0]
    prev_i = I[0]
    prev_r = R[0]

    curr_s = 0
    curr_i = 0
    curr_r = 0

    dtime = 0.0

    i = 1
    while i < steps:
        curr_s = prev_s + (-s_to_i(prev_s, prev_i)) * sim_step
        curr_i = prev_i + (s_to_i(prev_s, prev_i) - i_to_r(prev_i)) * sim_step
        curr_r = prev_r + i_to_r(prev_i) * sim_step

        if curr_s < 0:
            curr_s = 0.0

        if curr_i < 0:
            curr_i = 0.0

        dtime += sim_step
        if (dtime > precision):
            S[i] = curr_s;
            I[i] = curr_i;
            R[i] = curr_r;

            dtime -= precision;
            i += 1

        prev_s = curr_s
        prev_i = curr_i
        prev_r = curr_r

    return SimResult(S, I, R, T, i)


def write_to_file(filename, res):
    with open(filename, "w") as csvfile:
        wr = csv.writer(csvfile, delimiter=",", quotechar="\"", quoting = csv.QUOTE_MINIMAL)

        for i in range(0, res.len):
            wr.writerow([res.t[i], res.S[i], res.I[i], res.R[i]])
