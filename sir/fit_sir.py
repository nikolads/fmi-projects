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
        return 0.5*J

    def cost(self, beta):
        self.sir.infect_prob = beta
        sir_results = self.sir.simulate_points(self.time, sim_step=self.step)
        return self.error(sir_results.I)

    def cost_derivative(self, beta):
        return (self.cost(beta + 0.00001) - self.cost(beta)) / 0.00001

    def fit(self, beta, step):
        old_beta = 10
        while abs(beta - old_beta) > self.precision:
            old_beta = beta
            beta = old_beta - step * self.cost_derivative(old_beta)
        return beta
