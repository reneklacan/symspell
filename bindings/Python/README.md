## Symspell


## Quick examples using Python:

```python
>>> from symspell_rs import SymspellPy
>>> sym_spell = SymspellPy(max_distance=2,prefix_length=7,count_threshold=1)
>>> if not sym_spell.load_dictionary("./data/frequency_dictionary_en_82_765.txt",0,1," "):
      print("File Not Found")
>>> suggestions = sym_spell.lookup_compound("whereis th elove hehad dated forImuch of thepast who couqdn'tread in sixtgrade and ins pired him",2)
>>> for cand in suggestions:
    print(f"Term->{cand.term} \n Distance->{cand.distance} \n Count->{cand.count}")
```
## Bindings
  - [Python]()
