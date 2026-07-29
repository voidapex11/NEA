import pygame
import nea
from menu import MenuState
from constants import SETTING_FILE_PATH
from settings import SettingManager

class Program:
    def __init__(self):
        pass

    def run(self):

        # Initialize Pygame
        pygame.init()
        clock = pygame.time.Clock()
        self.screen = pygame.display.set_mode((1920/2, 1200/2))
        pygame.display.set_caption("Ant simulation")

        self.settings = SettingManager(SETTING_FILE_PATH)

        self.state = MenuState(self)

        # Game loop
        self.running = True
        while self.running:
                try:
                        self.events = pygame.event.get()
                        self.state.tick(self)
                        for event in self.events:
                                if event.type == pygame.QUIT:
                                        self.running = False
                                        self.settings.save(SETTING_FILE_PATH)
                        
                        pygame.display.update()
                        clock.tick(60)
                except Exception as e:
                        self.settings.save(SETTING_FILE_PATH)
                        raise e

        # Quit Pygame
        pygame.quit()

app_instance = Program()
app_instance.run()