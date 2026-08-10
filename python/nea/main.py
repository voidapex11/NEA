import pygame
from constants import SETTING_FILE_PATH
from menu import MenuState
from settings import SettingManager


class Program:
        """Main class for the program. It manages the main loop,
        getting events, handling the screen, the clock, and the
        settings."""

        def __init__(self):
                pass

        def check_if_quit(self):
                for event in self.events:
                        if event.type == pygame.QUIT:
                                self.running = False
                                self.settings.save(SETTING_FILE_PATH)

        def run(self):

                # Initialize Pygame
                pygame.init()
                clock = pygame.time.Clock()
                self.screen = pygame.display.set_mode(
                        (1920 / 2, 1200 / 2)
                )
                pygame.display.set_caption("Ant simulation")

                # Initialize settings
                self.settings = SettingManager(SETTING_FILE_PATH)

                # initialize main menu
                self.state = MenuState(self)

                # Game loop
                self.running = True
                while self.running:
                        try:
                                self.events = pygame.event.get()

                                self.state.tick(self)
                                self.check_if_quit()

                                pygame.display.update()
                                clock.tick(60)
                        except:
                                self.settings.save(SETTING_FILE_PATH)
                                raise

                # Quit Pygame
                pygame.quit()


app_instance = Program()
app_instance.run()
