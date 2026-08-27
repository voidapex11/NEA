"""description"""

import itertools
import math

import pygame
from constants import *
from state import State


def draw_rect(surface, colour, rect):
        pygame.draw.rect(surface, colour, rect)


def points_to_rect(point_a, point_b):
        start = (
                min(point_a[0], point_b[0]),
                min(point_a[1], point_b[1]),
        )
        end = (
                max(point_a[0], point_b[0]),
                max(point_a[1], point_b[1]),
        )
        dimentions = (end[0] - start[0], end[1] - start[1])
        return pygame.Rect(start, dimentions)


class Ant:
        def __init__(self, x, y):
                self.x = x
                self.y = y

        def draw(self, surface):
                screen_width, screen_height = SCREEN_DIMENTIONS
                width = screen_width / WIDTH
                height = screen_height / HEIGHT
                rect = pygame.Rect(
                        width * self.x + SMALL,
                        height * self.y + SMALL,
                        width - 3 * SMALL,
                        height - 3 * SMALL,
                )
                draw_rect(surface, BROWN, rect)

        def __eq__(self, other):
                return (self.x == other.x) and (self.y == other.y)


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
                        width * self.x - SMALL,
                        height * self.y - SMALL,
                        width + SMALL,
                        height + SMALL,
                )
                draw_rect(surface, DARK_GREY, rect)

        def to_raw(self):
                return (self.x, self.y)

        def __eq__(self, other):
                return (self.x == other.x) and (self.y == other.y)


class RenderState(State):
        def __init__(self, program):
                # todo
                self.tool = 0
                self.settings = program.settings
                program.walls = [
                        Wall(wall) for wall in program.walls
                ]

        def tick(self, program):
                self.draw(program)
                for event in program.events:
                        self.process_event(program, event)

                if self.tool == 0:
                        if pygame.mouse.get_pressed()[0]:
                                self.tick_draw_tool(program)
                        elif pygame.mouse.get_pressed()[2]:
                                self.tick_erase_tool(program)
                elif self.tool == 3:
                        if pygame.mouse.get_pressed()[0]:
                                self.tick_draw_ant_tool(program)
                        elif pygame.mouse.get_pressed()[2]:
                                self.tick_erase_ant_tool(program)

                if pygame.mouse.get_pressed()[0]:
                        if self.tool == 1:
                                self.tick_draw_tool(program)
                        elif self.tool == 2:
                                self.tick_erase_tool(program)

        def process_event(self, program, event):
                if event.type == pygame.MOUSEWHEEL:
                        self.tool = max(
                                min(3, self.tool - event.y), 0
                        )
                elif event.type == pygame.KEYDOWN:
                        if event.key == pygame.K_EQUALS:
                                program.settings.set_by_name(
                                        "radius",
                                        program.settings.get_by_name(
                                                "radius"
                                        )
                                        + 1,
                                )
                        elif event.key == pygame.K_MINUS:
                                program.settings.set_by_name(
                                        "radius",
                                        program.settings.get_by_name(
                                                "radius"
                                        )
                                        - 1,
                                )

        def get_affected_by_tool(self, program):
                x, y = self.get_mouse_grid_ref()
                radius = program.settings.get_by_name("radius")
                lower_x = x - math.floor(radius / 2)
                upper_x = x + math.ceil(radius / 2)
                lower_y = y - math.floor(radius / 2)
                upper_y = y + math.ceil(radius / 2)

                x_list = range(lower_x, upper_x)
                y_list = range(
                        lower_y,
                        upper_y,
                )
                return itertools.product(x_list, y_list)

        def get_mouse_grid_ref(self):
                x, y = pygame.mouse.get_pos()
                box_x = round(x / SCREEN_DIMENTIONS[0] * WIDTH)
                box_y = round(y / SCREEN_DIMENTIONS[1] * HEIGHT)
                return box_x, box_y

        def tick_draw_tool(self, program):
                for x, y in self.get_affected_by_tool(program):
                        wall = Wall(x, y)
                        if wall not in program.walls:
                                program.walls.append(wall)

        def tick_erase_tool(self, program):
                for x, y in self.get_affected_by_tool(program):
                        wall = Wall(x, y)
                        if wall in program.walls:
                                program.walls.remove(wall)

        def tick_draw_ant_tool(self, program):
                x, y = self.get_mouse_grid_ref()
                ant = Ant(x, y)
                if ant not in program.ants:
                        program.ants.append(ant)

        def tick_erase_ant_tool(self, program):
                x, y = self.get_mouse_grid_ref()
                ant = Ant(x, y)
                if ant in program.ants:
                        program.ants.remove(ant)

        def draw(self, program):
                # background
                program.screen.fill((255, 255, 255))

                for wall in program.walls:
                        wall.draw(program.screen)

                for ant in program.ants:
                        ant.draw(program.screen)

                self.draw_preview(program)

                pygame.display.flip()

        def draw_preview(self, program):
                surface = program.screen
                x, y = self.get_mouse_grid_ref()

                radius = program.settings.get_by_name("radius")

                if self.tool == 3:
                        radius = 1

                screen_width, screen_height = SCREEN_DIMENTIONS
                width = screen_width / WIDTH
                height = screen_height / HEIGHT
                lower_x = max(x - math.floor(radius / 2), 0) * width
                upper_x = (
                        min(x + math.ceil(radius / 2), WIDTH) * width
                )
                lower_y = max(y - math.floor(radius / 2), 0) * height
                upper_y = (
                        min(y + math.ceil(radius / 2), HEIGHT)
                        * height
                )

                points = (
                        (lower_x, lower_y),
                        (lower_x, upper_y),
                        (upper_x, upper_y),
                        (upper_x, lower_y),
                )

                point_pairs = (
                        (points[0], points[1]),
                        (points[1], points[2]),
                        (points[2], points[3]),
                        (points[3], points[0]),
                )

                for point_pair in point_pairs:
                        rect = points_to_rect(
                                point_pair[0], point_pair[1]
                        )
                        rect.x -= SMALL
                        rect.y -= SMALL
                        rect.w += SMALL * 2
                        rect.h += SMALL * 2
                        pygame.draw.rect(surface, BLACK, rect)
