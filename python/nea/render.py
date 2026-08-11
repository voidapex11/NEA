"""description"""

import pygame
from constants import *
from state import State


def draw_rect(surface, colour, rect):
        pygame.rect.draw_rect(surface,colour,rect)


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
                        width * self.x-SMALL,
                        height * self.y-SMALL,
                        width+SMALL,
                        height+SMALL,
                )
                pygame.draw.rect(surface, DARK_GREY, rect)

        def to_raw(self):
                return (self.x, self.y)


class RenderState(State):
        def __init__(self, program):
                # todo
                self.settings = program.settings
                program.walls = [
                        Wall(wall) for wall in program.walls
                ]

        def tick(self, program):
                self.draw(program)

                if pygame.mouse.get_pressed()[0]:
                        x, y = pygame.mouse.get_pos()
                        box_x = round(x/SCREEN_DIMENTIONS[0]*WIDTH)
                        box_y = round(y/SCREEN_DIMENTIONS[1]*HEIGHT)
                        wall = Wall(box_x,box_y)
                        if wall not in program.walls:
                                program.walls.append(wall)

        def draw(self, program):
                # background
                program.screen.fill((255, 255, 255))

                for wall in program.walls:
                        wall.draw(program.screen)

                for ant in program.ants:
                        ant.draw(program.screen)

                pygame.display.flip()
