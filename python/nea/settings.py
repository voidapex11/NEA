"""
This is the settings module.

It provides utilitys relating to settings for my program.
"""

from pathlib import Path
from enum import Enum
import json, copy, os
from constants import EXPECTED_CONFIG_ID

class SettingType(Enum):
        """Enum of the different types of setting
        >>> print(SettingType.DOUBLE_BOUNDED_INT)
        SettingType.DOUBLE_BOUNDED_INT
        >>> print(SettingType.DOUBLE_BOUNDED_INT.name)
        DOUBLE_BOUNDED_INT
        >>> print(SettingType.DOUBLE_BOUNDED_INT.value)
        3
        """
        UNBOUNDED_INT = 0
        LOWER_BOUND_INT = 1
        UPPER_BOUND_INT = 2
        DOUBLE_BOUNDED_INT = 3
        FILE_PATH = 4
        SETTING_FILE_PATH = 5
        

class SettingManager:
        def __init__(self,fp):
                with open(fp,"r") as file:
                        data = json.load(file)
                        self.categorys = data["categorys"]
                        self.settings = [Setting.from_raw_data(setting) for setting in data["settings"]]
                        
        
        def save(self, fp):
                data = {
                        "categorys": self.categorys,
                        "settings": self.settings
                        }
                for setting,i in zip(data["settings"],range(len(data["settings"]))):
                        #import pdb;pdb.set_trace()
                        data["settings"][i]=self.settings[i].to_raw_data()
                
                with open(fp,"w") as file:
                        json.dump(data, file,indent=8)
        
        def get_all_from_category(self, cat):
                return [
                        self.settings[setting]
                        for setting in self.settings
                        if setting in self.categorys[cat]
                        ]

class Setting:
        """Reppresents a setting.
        >>> fp = Setting(SettingType.FILE_PATH,"~/Downloads/test.txt")
        >>> fp.encode()
        '{"type": "FILE_PATH", "data": ["~/Downloads/test.txt"]}'
        >>> fp.validate()
        True

        valid case
        >>> age = Setting(SettingType.LOWER_BOUND_INT, 2, 0)
        >>> age.validate()
        True
        
        invalid case of negative age
        >>> invalid_age = Setting(SettingType.LOWER_BOUND_INT, -2, 0)
        >>> invalid_age.validate()
        False

        
        >>> before = Setting(SettingType.LOWER_BOUND_INT, 2, 0)
        >>> after = Setting.decode(before.encode())
        >>> before == after
        True
        """
        def __init__(self, type: SettingType, *args, data=[]):
                self.type = type
                self.data = list(args) + data

        def __eq__(self, other): 
                """https://stackoverflow.com/questions/1227121/compare-object-instances-for-equality-by-their-attributes"""
                if not isinstance(other, Setting):
                        # don't attempt to compare against unrelated types
                        return NotImplemented

                return self.type == other.type and self.data == other.data

        def get_value(self):
                return self.data[0]

        def encode(self):
                data = self.to_raw_data()
                return json.dumps(data)
        
        def to_raw_data(self):
                data = copy.deepcopy(self.__dict__)
                data["type"] = self.type.name
                return data

        def decode(raw):
                decoded_json = json.loads(raw)
                decoded = Setting.from_raw_data(decoded_json)
                return decoded
        
        def from_raw_data(data):
                data["type"] = SettingType[data["type"]]
                decoded = Setting(**data)
                return decoded
        
        def validate(self):
                """calls the setting type spesific validator function"""
                return validations[self.type](*self.data)

        def validate_bound_int(data: int, bound, upper_bound: bool):
                if upper_bound:
                        return data <= bound
                else:
                        return bound <= data

        def validate_config(fp):
                """Validates config should be made for this program
                via use of a fixed ID that is large enough that 
                the file must have been made with pre knowlege of
                the id."""
                with open(fp,"r") as file:
                        data = json.load(file)
                        return EXPECTED_CONFIG_ID in data

        def validate_fp(fp):
                """Validation function for a savefile
                
                Prevent boundary invalid case. 
                >>> import pathlib
                >>> Setting.validate_fp(pathlib.Path.home().__str__())
                False"""
                home = Path.home()
                target_path = Path(os.path.realpath(fp))
                
                if os.path.isfile(fp):
                        return False
                

                return home in target_path.parents



validations = {
        SettingType.UNBOUNDED_INT: lambda: True,
        SettingType.LOWER_BOUND_INT: lambda *args:
                Setting.validate_bound_int(*args,upper_bound=False),
        SettingType.UPPER_BOUND_INT: lambda *args:
                Setting.validate_bound_int(*args,upper_bound=True),
        SettingType.DOUBLE_BOUNDED_INT: lambda data,lower,upper:
                Setting.validate_bound_int(data,lower,upper_bound=False)
                and Setting.validate_bound_int(data,upper,upper_bound=True),
        SettingType.FILE_PATH: lambda path:
                Setting.validate_fp(path),

}

if __name__ == "__main__":
    import doctest
    doctest.testmod()