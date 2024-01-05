__author__ = "Mohamed Ali MEZNI"
__version__ = "2024-01-05"

from cost_core import Settings

settings, status = Settings().get_settings()

print(settings)
