import aro
import aro
print(aro.salutation())
assert aro.multiplier(3, 4) == 12  # Test silencieux si réussi

def test_aro():
    print(aro.salutation())
    assert aro.multiplier(3, 4) == 12

if __name__ == "__main__":
    test_aro()
