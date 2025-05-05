import aro
import unittest

class TestAro(unittest.TestCase):
    def test_functions(self):
        self.assertIn("Aro", aro.salutation())
        self.assertEqual(aro.multiplier(3, 4), 12)

if __name__ == "__main__":
    unittest.main()
