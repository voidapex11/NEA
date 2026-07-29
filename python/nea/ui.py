from constants import *
import pygame

class Button:
        def __init__(self,text, position: pygame.Rect):
                self.text = text
                self.hooks = {"mouse": []}
                
                self.outer_rect = position
                self.text_rect = position.move(BORDER_HEIGHT,BORDER_HEIGHT)
                self.text_rect.clip(position)

                # temporary demmo hook
                # self.add_clicked_on_hook(print,(f"click on \"{text}\"",))
                
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

class TextBox:
        def __init__(self, position: pygame.Rect, program, text=""):
                self.text = text
                self.program = program
                self.active = False
                self.outer_rect = position
                self.width = position.width
                self.text_rect = position.move(BORDER_HEIGHT,BORDER_HEIGHT)
                self.text_rect.clip(position)

        def draw(self, surface):
                if self.active or self.outer_rect.collidepoint(pygame.mouse.get_pos()):
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
                self.outer_rect.w = max(self.width,text.get_width()+BORDER_HEIGHT)

        def tick(self):
                if 0!=max(pygame.mouse.get_pressed()):
                        if self.outer_rect.collidepoint(pygame.mouse.get_pos()):
                                self.active = True
                        else:
                                self.active = False
                for event in self.program.events:        
                        if event.type == pygame.KEYDOWN:
                                if event.key == pygame.K_BACKSPACE:
                                        self.text = self.text[:-1]
                                else:
                                        self.text+=event.unicode