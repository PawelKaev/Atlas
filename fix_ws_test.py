# fix_ws_test.py
content = open('ide/src/websocket.rs', 'r', encoding='utf-8').read()
content = content.replace(
    'assert!(message.message.to_string().contains("test_synthesis"));',
    'assert!(message.data.message.contains("test_synthesis"));'
)
open('ide/src/websocket.rs', 'w', encoding='utf-8').write(content)
print('Fixed')
