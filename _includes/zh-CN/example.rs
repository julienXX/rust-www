// ��������ǿ��Ա༭�����ܹ����е�
fn main() {
    // һ�����׼�����
    // `+` �� `-` ��ζ�żӼ�1
    // `*` �� `/` ��ζ�ų˳�2

    let program = "+ + * - /";
    let mut accumulator = 0;

    for token in program.chars() {
        match token {
            '+' => accumulator += 1,
            '-' => accumulator -= 1,
            '*' => accumulator *= 2,
            '/' => accumulator /= 2,
            _ => { /* �������� */ }
        }
    }

    println!("���� \"{}\" �Ľ���� {}",
              program, accumulator);
}
