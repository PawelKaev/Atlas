# update_lib_v09.py
content = open('grammalang-core/src/lib.rs', 'r', encoding='utf-8').read()
if 'pub mod reflexive' not in content:
    content = content.replace('pub mod social;', 'pub mod social;\npub mod reflexive;')
    open('grammalang-core/src/lib.rs', 'w', encoding='utf-8').write(content)
    print('Added pub mod reflexive')
else:
    print('Already exists')
