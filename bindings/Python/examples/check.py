import symspell_rs as sym
# print(sym.countline())
# # from symspell_rs as sym
# import time
# print(SymspellPy(max_distance=2,prefix_length=7,count_threshold=1))
sym_spell = sym.SymspellPy(max_distance=2,prefix_length=7,count_threshold=1)
# #     # {"max_edit_distance":2,"prefix_length1":,"count_threshold":1})
# # # sym_spell = SymspellPy()
if not sym_spell.load_dictionary("../../data/frequency_dictionary_en_82_765.txt",0,1," "):
    print("Not Found")
else:
    print("Found")
# start = time.time()
suggestions = sym_spell.lookup_compound("whereis th elove hehad dated forImuch of thepast who couqdn'tread in sixtgrade and ins pired him",2)
# # # suggestions = sym_spell.lookup("roet",0,2)
# # # words = sym_spell.get_words()
# # # print(len(words))
for cand in suggestions:
    print(f"Term->{cand.term}\nDistance->{cand.distance}\nCount->{cand.count}")

output = sym_spell.word_segmentation("whereisthelove",2)
print(f"String->{output.segmented_string}\nDistance->{output.distance_sum}\nProb_Log_Sum->{output.prob_log_sum}")
# print(time.time()-start)
