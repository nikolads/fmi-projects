from __future__ import division
import numpy as np
import math
import csv

class FitSir:
    def __init__(self, sir, data, time, step, precision):
        self.sir = sir
        self.data = data
        self.time = time
        self.step = step
        self.precision = precision

    def error(self, y):
        J = 0
        for i in range(0, len(self.data)):
            J += (self.data[i] - y[i])**2
        return J

    def cost(self, beta):
        self.sir.infect_prob = beta
        sir_results = self.sir.simulate_points(self.time, sim_step=self.step)
        return self.error(sir_results.I)

    def cost_derivative(self, beta):
        return (self.cost(beta + 0.000001) - self.cost(beta)) / 0.000001

    def fit(self, beta, step):
        curr_step = 0

        while True:
            curr_cost = self.cost_derivative(beta)
            beta = beta - step * curr_cost

            curr_step = curr_step + 1
            if curr_step == 1000:
                print("Simulating... beta={}".format(beta));
                curr_step = 0

            if abs(curr_cost) < self.precision:
                break

        return beta
