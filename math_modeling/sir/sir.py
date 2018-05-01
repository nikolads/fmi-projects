from __future__ import division
import numpy as np
import math
import csv

class Sir:
    def __init__(self, s0, i0, r0, infect_prob, recovery_rate):
        self.s0 = s0
        self.i0 = i0
        self.r0 = r0
        self.infect_prob = infect_prob
        self.recovery_rate = recovery_rate

    # Make a simulation and return data for time period [0; sim_time]
    def simulate(self, sim_time, **kwargs):
        precision = kwargs.pop("precision", 1.0)
        sim_step = kwargs.pop("sim_step", 0.01)

        steps = int(math.ceil(sim_time / precision))

        S = np.zeros(steps)
        I = np.zeros(steps)
        R = np.zeros(steps)
        T = np.linspace(0, sim_time, steps)

        S[0] = float(self.s0)
        I[0] = float(self.i0)
        R[0] = float(self.r0)

        def s_to_i(prev_s, prev_i):
            return self.infect_prob * prev_s * prev_i

        def i_to_r(prev_i):
            return self.recovery_rate * prev_i

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

        return SimResult(S, I, R, T)


    # Make a simulation and return data for a given set of time points
    def simulate_points(self, time_points, **kwargs):
        sim_step = kwargs.pop("sim_step", 0.01)

        steps = len(time_points)

        S = np.zeros(steps)
        I = np.zeros(steps)
        R = np.zeros(steps)

        def s_to_i(prev_s, prev_i):
            return self.infect_prob * prev_s * prev_i

        def i_to_r(prev_i):
            return self.recovery_rate * prev_i

        prev_s = float(self.s0)
        prev_i = float(self.i0)
        prev_r = float(self.r0)

        curr_s = 0
        curr_i = 0
        curr_r = 0

        total_time = 0.0

        i = 0
        while i < steps:
            curr_s = prev_s + (-s_to_i(prev_s, prev_i)) * sim_step
            curr_i = prev_i + (s_to_i(prev_s, prev_i) - i_to_r(prev_i)) * sim_step
            curr_r = prev_r + i_to_r(prev_i) * sim_step

            if curr_s < 0:
                curr_s = 0.0

            if curr_i < 0:
                curr_i = 0.0

            total_time += sim_step
            if total_time > time_points[i]:
                S[i] = prev_s
                I[i] = prev_i
                R[i] = prev_r
                i += 1

            prev_s = curr_s
            prev_i = curr_i
            prev_r = curr_r

        return SimResult(S, I, R, time_points)


class SimResult:
    def __init__(self, S, I, R, time):
        self.S = S
        self.I = I
        self.R = R
        self.t = time

    def write_to_file(self, filename):
        with open(filename, "w") as csvfile:
            wr = csv.writer(csvfile, delimiter=",", quotechar="\"", quoting = csv.QUOTE_MINIMAL)

            for i in range(0, len(self.t)):
                wr.writerow([self.t[i], self.S[i], self.I[i], self.R[i]])
