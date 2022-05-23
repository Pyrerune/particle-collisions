from manim import *
import numpy as np
class Presentation(Scene):
    def construct(self):
        circle_a = Circle(5.0)
        circle_b = Circle(5.0)
        circle_a.shift(np.array([5.0, 5.0, 0.0]))
        self.play(Create(circle_a))
        self.play(Create(circle_b))

