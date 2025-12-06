"""
Shannon Entropy calculation for data uncertainty assessment.

[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]
"""

import math
from typing import List, Dict


def calculate_entropy(data_blocks: List[str]) -> float:
    """
    Calculates the Shannon Entropy (H) of the concatenated input data based on
    character frequencies. H = -sum(pk * log2(pk)).
    
    Data blocks are first joined into a single string for frequency analysis.
    """
    if not data_blocks:
        return 0.0

    source_string = "".join(data_blocks)
    text_len = len(source_string)

    if text_len == 0:
        return 0.0

    # 1. Count character frequencies (Histogram)
    frequency: Dict[str, int] = {}
    for char in source_string:
        frequency[char] = frequency.get(char, 0) + 1

    # 2. Calculate the entropy sum
    entropy_sum = 0.0
    for count in frequency.values():
        # Probability pk
        prob = count / text_len
        
        # Calculate the term: -pk * log2(pk)
        # We use log(x) / log(2) as a standard way to calculate log base 2
        # (Using math.log2 is also acceptable in modern Python, but we adhere to
        # the structure derived from pure Python examples for base 2 calculation principle.)
        term = prob * math.log(prob, 2)
        entropy_sum -= term 
        
    return entropy_sum

