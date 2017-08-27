#ifndef BRACKET_PUSH_H
#define BRACKET_PUSH_H

#include <string>
#include <stack>

namespace bracket_push {
    static inline bool is_matched(std::stack<char> &v, char expected) {
        if (v.empty() || v.top() != expected) {
            return false;
        }

        v.pop();

        return true;
    }

    static inline bool check(std::string input) {
        char expected;
        std::stack<char> v;

        for (auto &c : input) {
            switch (c) {
                case '[':
                case '{':
                case '(':
                    v.push(c);
                    break;

                case ']':
                    expected = '[';
                    if (!is_matched(v, expected)) {
                        return false;
                    }
                    break;

                case '}':
                    expected = '{';
                    if (!is_matched(v, expected)) {
                        return false;
                    }
                    break;

                case ')':
                    expected = '(';
                    if (!is_matched(v, expected)) {
                        return false;
                    }
                    break;

                default:
                    break;
            }
        }

        return v.empty();
    }
}

#endif//BRACKET_PUSH_H
