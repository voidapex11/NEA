from state import State
from constants import *
import pygame
# import random

def announce():
        print()

class MenuItem:
        def __init__(self,text, position: pygame.Rect):
                self.text = text
                self.hooks = {"mouse": []}
                
                self.outer_rect = position
                self.text_rect = position.move(BORDER_HEIGHT,BORDER_HEIGHT)
                self.text_rect.clip(position)
                self.add_clicked_on_hook(print,("click on",text))
                
                # self.text_rect.scale_by_ip(RECT_WIDTH,RECT_HEIGHT-BORDER_HEIGHT*2)
        
        def draw(self,surface):
                if self.outer_rect.collidepoint(pygame.mouse.get_pos()):
                        self.colour = DARK_GREY
                else:
                        self.colour = LIGHT_GREY
                pygame.draw.rect(surface,self.colour,self.outer_rect)
                font = pygame.font.Font(None, FONT_HEIGHT)
                text = font.render(self.text,True,(0,0,0))

                surface.blit(
                        text,
                        self.text_rect
                        )

        def add_clicked_on_hook(self, hook, *args):
                self.hooks["mouse"].append([hook,args])
        
        def clicked_on(self):
                if pygame.mouse.get_pressed()[0] and self.outer_rect.collidepoint(pygame.mouse.get_pos()):
                        return True
                else:
                        return False

        def process_hooks(self):
                if self.clicked_on():
                        for hook, args in self.hooks["mouse"]:
                                
                                hook(*(args[0]))



        
        

class MenuState(State):
        def __init__(self,program):
                self.items = []
                self.register_menu_item("1")
                self.register_menu_item("2")
                self.register_menu_item("3")
                
                # todo: load settings
                pass

        def register_menu_item(self,text):
                self.items.append(MenuItem(
                        text,
                        pygame.Rect(
                                RECT_WIDTH/2,
                                RECT_HEIGHT*(2+1.25*len(self.items)),
                                RECT_WIDTH,
                                RECT_HEIGHT
                                )
                                ))

        def draw(self, program):
                # background
                program.screen.fill((255,255,255))

                for item,i in zip(self.items,range(len(self.items))):
                        item.draw(program.screen)

                # todo: draw settings

                pass

        def draw_item(item: MenuItem):
                pass

        def tick(self, program):
                self.draw(program)
                for item in self.items:
                        item.process_hooks()
                # process events for buttons
                pass

