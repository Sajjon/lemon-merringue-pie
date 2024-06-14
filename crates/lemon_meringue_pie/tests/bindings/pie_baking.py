
import unittest

from common import *
from chef import *
from money import *

class TestCoverall(unittest.TestCase):

    def test_baking(self):
        chef = Chef("August Gusteau", Money(50))
                         
if __name__=='__main__':
    unittest.main()