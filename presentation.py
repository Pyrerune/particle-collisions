import math

from manim import *
import numpy as np


class Presentation(Scene):
    def construct(self):
        circle_a = Circle(1.0)
        circle_b = circle_a.copy().shift(LEFT * 2)
        circle_b.rotate(PI / 8, about_point=circle_a.get_center())
        normal_vector = circle_b.get_center() - circle_a.get_center()
        normal = Line(circle_b.get_center(), circle_a.get_center())

        normal_brace = Brace(normal, direction=normal.copy().rotate(PI / 2).get_unit_vector())
        normal_label = normal_brace.get_text("Normal")

        tangent = normal.copy().set_angle(normal.get_angle()).rotate(PI / 2).move_to(normal.get_center())
        tangent_brace = Brace(tangent, direction=tangent.copy().rotate(PI / 2).get_unit_vector())
        tangent_label = tangent_brace.get_text("Tangent")

        self.play(Create(circle_a))
        self.play(Create(circle_b))

        circle_a.add(Dot(circle_a.get_center()))
        circle_b.add(Dot(circle_b.get_center()))

        self.play(Create(normal))
        circle_a.add(normal.copy())
        self.play(FadeIn(normal_brace, normal_label))

        self.pause(0.5)
        self.play(FadeOut(normal_brace, normal_label))
        self.play(FadeOut(circle_a, circle_b))
        self.play(normal.animate.set_angle(0.0))

        normal_label = Text(str([normal_vector[0].round(2), normal_vector[1].round(2)])).scale(0.5)
        normal_label.add(Text("Normal").scale(0.5).next_to(normal_label, LEFT))
        normal_label = normal_label.next_to(normal, UP)

        self.play(FadeIn(normal_label))

        unit_normal = np.array(normal_vector) / math.sqrt(math.pow(normal_vector[0], 2) + math.pow(normal_vector[1], 2))

        self.play(FadeOut(normal_label))

        normal_label = Text(str([unit_normal[0].round(2), unit_normal[1].round(2)])).scale(0.5)
        normal_label.add(Text("Unit Normal").scale(0.5).next_to(normal_label, LEFT))
        normal_label = normal_label.next_to(normal, UP)

        self.play(FadeIn(normal_label), normal.animate.scale(0.5))

        unit_tangent = normal.copy().rotate(PI / 2).move_to((3.0, 3.0, 0.0)).shift(DOWN * 2)
        normal = normal.add(normal_label)

        self.play(normal.animate.move_to((3.0, 3.0, 0.0)))

        self.play(FadeIn(circle_a, circle_b))
        self.play(Create(tangent))
        circle_a.add(tangent)
        self.play(FadeIn(tangent_brace))
        self.play(FadeIn(tangent_label))
        self.wait(0.5)
        unit_tangent.add(Text("Unit Tangent").next_to(unit_tangent, UP).scale(0.5))
        self.play(FadeOut(circle_a, circle_b, tangent, tangent_brace, tangent_label))
        self.play(FadeIn(unit_tangent))
        scene = Mobject()
        scene.add(unit_tangent, normal)
        self.play(scene.animate.scale(0.5).shift(RIGHT * 2))
        self.wait(1.5)



        v1f = MathTex("\\overrightarrow{V^{'}_1} = \\overrightarrow{V^{'}_{1n}} + \\overrightarrow{V^{'}_{1t}}")
        v2f = MathTex("\\overrightarrow{V^{'}_2} = \\overrightarrow{V^{'}_{2n}} + \\overrightarrow{V^{'}_{2t}}")
        v1f.add(v2f.next_to(v1f, DOWN))

        normal_eq = normal_eq_desc(self)
        v1n = scalar_desc(self, normal_eq)
        primes_desc_a(self, v1n)
        v2 = primes_desc_b(self, normal_eq)

        self.play(FadeIn(v1f))
        self.wait(3.0)
        self.play(v1f.animate.next_to(v2, DOWN).scale(0.5))


def primes_desc_b(scene, normal_eq):
    v1 = MathTex("\\overrightarrow{V^{'}_{1n}} = V^{'}_{1n} * \\overrightarrow{un}")
    v1.add(MathTex("\\overrightarrow{V^{'}_{1t}} = V^{'}_{1t} * \\overrightarrow{ut}").next_to(v1, DOWN))
    v2 = MathTex("\\overrightarrow{V^{'}_{2n}} = V^{'}_{2n} * \\overrightarrow{un}")
    v2.add(MathTex("\\overrightarrow{V^{'}_{2t}} = V^{'}_{2t} * \\overrightarrow{ut}").next_to(v2, DOWN))
    scene.play(FadeIn(v1))
    scene.wait(3.0)
    scene.play(v1.animate.next_to(normal_eq, RIGHT).scale(0.5))
    scene.play(FadeIn(v2))
    scene.wait(3.0)
    scene.play(v2.animate.next_to(v1, DOWN).scale(0.5))
    return v2


def primes_desc_a(scene, v1n):
    v1PrimeT = MathTex("V^{'}_{1t} = v_{1t}")
    v2PrimeT = MathTex("V^{'}_{2t} = v_{2t}")
    v1PrimeT.add(v2PrimeT.next_to(v1PrimeT, DOWN))
    v1PrimeN = MathTex("V^{'}_{1n} = \\frac{v_{1n}(m_1-m_2)+2(m_2*v_{2n})}{m_1+m_2}")
    v2PrimeN = MathTex("V^{'}_{2n} = \\frac{v_{2n}(m_2-m_1)+2(m_1*v_{1n})}{m_1+m_2}")
    scene.play(FadeIn(v1PrimeT))
    scene.play(v1PrimeT.animate.next_to(v1n, DOWN).scale(0.5))
    scene.play(FadeIn(v1PrimeN))
    scene.play(v1PrimeN.animate.next_to(v1PrimeT, DOWN).scale(0.5))
    scene.play(FadeIn(v2PrimeN))
    scene.wait(3.0)
    scene.play(v2PrimeN.animate.next_to(v1PrimeN, DOWN).scale(0.5))
    scene.wait(3.0)


def scalar_desc(scene, normal_eq):
    v1n = MathTex("v_{1n} = \\overrightarrow{un}\\bullet\\overrightarrow{v_1}")
    v1t = MathTex("v_{1t} = \\overrightarrow{ut}\\bullet\\overrightarrow{v_1}")
    v1n.add(v1t.next_to(v1n, RIGHT))
    v2n = MathTex("v_{2n} = \\overrightarrow{un}\\bullet\\overrightarrow{v_2}")
    v2t = MathTex("v_{2t} = \\overrightarrow{ut}\\bullet\\overrightarrow{v_2}")
    v2n.add(v2t.next_to(v2n, RIGHT))
    v1n.add(v2n.next_to(v1n, DOWN))

    scene.play(FadeIn(v1n))
    scene.wait(1.0)
    scene.play(v1n.animate.next_to(normal_eq, DOWN).scale(0.5))
    return v1n


def normal_eq_desc(scene):
    equation = MathTex("\\overrightarrow{un} = \\frac{\\overrightarrow{n}}{\\sqrt{n^2_x + n^2_y}}")
    scene.play(FadeIn(equation))
    scene.wait(1.0)
    scene.play(equation.animate.move_to((-5.0, 3.0, 0.0)).scale(0.5))

    return equation
