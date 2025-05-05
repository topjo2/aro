# test_aro.py
import aro
import unittest

class TestAro(unittest.TestCase):
    def test_fonctions(self):
        self.assertEqual(aro.salutation(), "Bonjour, je suis Aro, votre assistante Rust! 🟡")
        self.assertEqual(aro.multiplier(3, 4), 12)
        self.assertEqual(aro.au_revoir(), "Au revoir, merci d'avoir utilisé Aro! 🟢")

if __name__ == "__main__":
    unittest.main()
