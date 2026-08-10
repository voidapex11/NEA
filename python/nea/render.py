"""description"""

import pygame
from state import State
from constants import *


class Wall:
        def __init__(self, *args):
                if len(args) == 1:
                        self.x = args[0][0]
                        self.y = args[0][1]
                elif len(args) == 2:
                        self.x = args[0]
                        self.y = args[1]
                else:
                        raise TypeError(
                                "invalid number of arguments"
                        )

        def draw(self, surface):
                screen_width, screen_height = SCREEN_DIMENTIONS
                width = screen_width / WIDTH
                height = screen_height / HEIGHT
                rect = pygame.Rect(
                        width * self.x,
                        height * self.y,
                        width,
                        height,
                )
                pygame.draw.rect(surface,DARK_GREY,rect)


class RenderState(State):
        def __init__(self, program):
                # todo
                self.settings = program.settings

        def tick(self, program):
                self.draw(program)

        def draw(self, program):
                for wall in program.walls:
                        wall.draw()

                for ant in program.ants:
                        ant.draw()

                pygame.display.flip()
